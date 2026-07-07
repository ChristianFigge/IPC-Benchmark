use ipc_benchmark::{mean, stddev};
use std::io::{Read, Write};
use std::process::{Command, Stdio};


// Predefine IPC message lengths to bench & number of samples to collect
// (maybe as CLI args for main binary?)
const MSG_SIZES: [usize; 5] = [
    8,
    16 * 1024,
    64 * 1024,
    256 * 1024,
    1024 * 1024,
];
const N_SAMPLES: usize = 500;

/** PIPE SPEED BENCH
 * Starts programs Tx, Rx and pipes Tx output into Rx.
 * Synchronizes the processes with ready/go-Signals
 * and then waits for the sample from Rx.
 * (The actual measurements are done by Tx & Rx)
*/
fn pipe_speed_bench(samples: &mut [u64; N_SAMPLES], msg_size: usize, build_profile: &str) -> () {
    let tx_path = format!("target/{build_profile}/tx_pipe");
    let rx_path = format!("target/{build_profile}/rx_pipe");
    let mut sample_buf = [0u8; 8];

    println!("Piping {} * {} Bytes ...(from {} to {})", N_SAMPLES, msg_size, tx_path, rx_path);
    for i in 0..N_SAMPLES {
        // Tx takes the msg size as a command line argument
        let mut tx = Command::new(tx_path.clone())
            .args(&[msg_size.to_string()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start Tx");

        let mut rx = Command::new(rx_path.clone())
            .stdin(tx.stdout.take().unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start Rx");

        // SYNCHRONIZE both processes:
        // Corrects the timestamps made by Tx/Rx, so the measured duration includes less overhead
        // 1. Wait for Rx "Ready" Signal
        let mut rx_out = rx.stdout.take().unwrap();
        rx_out
            .read_exact(&mut [0u8; 1])
            .expect("Failed to get Sync Signal from Rx");

        // 2. Send "GO" Signal to Tx, which waits for it
        let tx_in = tx.stdin.as_mut().unwrap();
        tx_in
            .write(&[1u8])
            .expect("Failed to send Sync Signal to Tx");

        // Wait for Rx to return a Sample & Save it in vector
        rx_out
            .read_exact(&mut sample_buf)
            .expect("Failed to read Rx Sample");
        samples[i] = u64::from_le_bytes(sample_buf);

        // Wait for both processes to finish
        let _tx_status = tx.wait().unwrap();
        let _rx_status = rx.wait().unwrap();

        // TODO panic if tx_stats != 0 && rx_status != 0
        //println!("tx exited with: {} \nrx exited with: {}", tx_status.code().unwrap(), rx_status.code().unwrap());
    }
}


fn handle_samples(samples: &[u64]) -> () {
    // Calculate mean, stddev, coefficient of variation (cv) and relative standard error (rse)
    let mean = mean(&samples).unwrap();
    let stddev = stddev(&samples, mean).unwrap();
    let cv = stddev / mean;
    let rse = cv / (samples.len() as f64).sqrt();

    eprintln!(
        "Mean: {mean:.1} cycles, StdDev: {stddev:.1} cycles -> CV = {cv:.3}, RSE = {rse:.3} \n"
    );

    // TODO log samples somewhere
}


fn hardware_is_supported() -> bool {
    #[cfg(not(target_arch = "x86_64"))]
    { false }

    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::x86_64::__cpuid;

        // Check if the relevant CPUID-Leafs are known at all
        let highest_extended_leaf = __cpuid(0x8000_0000).eax;
        if highest_extended_leaf < 0x8000_0007 {
            return false; // CPU is too old/virtualized/etc.
        }

        // Check RDTSCP support (CPUID.80000001H:EDX[27]):
        let leaf_1 = __cpuid(0x8000_0001);
        let rdtscp_bit = 1 << 27;
        let supports_rdtscp = (leaf_1.edx & rdtscp_bit) != 0;

        // Check invariant TSC support (CPUID.80000007H:EDX[8]):
        let leaf_7 = __cpuid(0x8000_0007);
        let invariant_tsc_bit = 1 << 8;
        let supports_invariant_tsc = (leaf_7.edx & invariant_tsc_bit) != 0;

        supports_rdtscp && supports_invariant_tsc
    }
}


fn main() {
    // Check hardware support for invariant TSC
    if !hardware_is_supported() {
        panic!("This benchmark requires an x86_64 CPU with RDTSCP and invariant TSC support!");
    }

    // Get current build profile to match the binary paths
    let build_profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let mut samples= [0u64; N_SAMPLES];

    // BENCHMARK PIPE SPEED
    for msg_size in MSG_SIZES.iter() {
        pipe_speed_bench(&mut samples, *msg_size, build_profile);
        handle_samples(&samples);
    }
}
