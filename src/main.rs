use std::io::{Read, Write};
use std::process::{Command, Stdio};
use ipc_benchmark::{to_microsecs};

// Predefine IPC message lengths to bench & samples to collect
// (maybe as CLI args for main binary?)
const MSG_SIZES: [usize; 1] = [8]; // Prototype test value
const N_SAMPLES: usize = 100;

/** PIPE SPEED BENCH
 * Starts programs Tx, Rx and pipes Tx output into Rx.
 * Synchronizes the processes with ready/go-Signals
 * and then waits for the sample from Rx.
*/
fn main() {
    // Get current build profile to match the paths to Rx/Tx binaries
    let build_profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let tx_path = format!("target/{build_profile}/tx_pipe");
    let rx_path = format!("target/{build_profile}/rx_pipe");
    let mut sample_buf = [0u8; 8];
    let mut samples : Vec<f64> = Vec::with_capacity(N_SAMPLES);

    // TODO iterate over MSG_SIZES...
    println!("Piping {} * {} Bytes ...", N_SAMPLES, MSG_SIZES[0]);
    for _ in 0..N_SAMPLES {
        // Tx takes the msg size as a command line argument
        let mut tx = Command::new(tx_path.clone())
            .args(&[MSG_SIZES[0].to_string()])
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
        rx_out.read_exact(&mut [0u8; 1])
            .expect("Failed to get Sync Signal from Rx");

        // 2. Send "GO" Signal to Tx, which waits for it
        let tx_in = tx.stdin.as_mut().unwrap();
        tx_in.write(&[1u8])
            .expect("Failed to send Sync Signal to Tx");

        // Wait for Rx to return a Sample & Save it in vector
        rx_out.read_exact(&mut sample_buf)
            .expect("Failed to read Rx Sample");
        samples.push(to_microsecs(u64::from_le_bytes(sample_buf)));

        // Wait for both processes to finish
        let _tx_status = tx.wait().unwrap();
        let _rx_status = rx.wait().unwrap();

        // TODO panic if tx_stats != 0 && rx_status != 0
        //println!("tx exited with: {} \nrx exited with: {}", tx_status.code().unwrap(), rx_status.code().unwrap());
    }

    eprintln!("Durations in μs: {:?}", samples);
    // TODO log samples somewhere
}
