use std::process::{Command, Stdio};
use std::time::Instant;
use ipc_benchmark::to_microsecs;

// Predefine IPC message lengths to bench
const MSG_SIZES: [usize; 1] = [100000000]; // Prototype test value

/** PIPE SPEED PENCH
 * Starts programs tx, rx and pipes tx output into rx.
 * Measures & outputs the lifecycle length of both processes.
 * A more precise measurement of the actual IPC performance is done by rx.
*/
fn main() {
    // Get current profile to match the paths to rx/tx binaries
    let build_profile = if cfg!(debug_assertions) { "debug" } else { "release" };

    let pipe_bench_starttime = Instant::now();

    // TODO iterate over MSG_SIZES...
    // TODO set cargo profile-dependend program paths (bench, release ...)
    // TX takes the msg size as a command line argument
    let mut tx = Command::new(format!("target/{}/tx_pipe", build_profile))
        .args(&[MSG_SIZES[0].to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start tx");
    
    let mut rx = Command::new(format!("target/{}/rx_pipe", build_profile))
        .stdin(tx.stdout.take().unwrap())
        .spawn()
        .expect("Failed to start rx");

    // Wait for both processes to finish
    let _tx_status = tx.wait().unwrap();
    let _rx_status = rx.wait().unwrap();
    
    let pipe_bench_duration = pipe_bench_starttime.elapsed();

    // TODO panic if tx_stats != 0 && rx_status != 0
    //println!("tx exited with: {} \nrx exited with: {}", tx_status.code().unwrap(), rx_status.code().unwrap());
    
    println!("The pipe bench (with execution overhead) took: {:.1} μs", to_microsecs(pipe_bench_duration.as_nanos() as u64));
}
