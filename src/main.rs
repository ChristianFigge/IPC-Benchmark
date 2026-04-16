use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::Instant;
use ipc_benchmark::to_microsecs;

// Predefine IPC message lengths to bench
const MSG_SIZES: [usize; 1] = [8]; // Prototype test value

/** PIPE SPEED PENCH
 * Starts programs tx, rx and pipes tx output into rx.
 * Measures & outputs the lifecycle length of both processes.
 * A more precise measurement of the actual IPC performance is done by rx.
*/
fn main() {
    // Get current profile to match the paths to rx/tx binaries
    let build_profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let tx_path = format!("target/{build_profile}/tx_pipe");
    let rx_path = format!("target/{build_profile}/rx_pipe");

    let pipe_bench_starttime = Instant::now();

    // TODO iterate over MSG_SIZES...
    // TX takes the msg size as a command line argument
    let mut tx = Command::new(tx_path)
        .args(&[MSG_SIZES[0].to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start tx");
    
    let mut rx = Command::new(rx_path)
        .stdin(tx.stdout.take().unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start rx");

    // SYNCHRONIZE both processes:
    // Affects timestamps made by tx/rx, so the measured duration includes less overhead
    // 1. Wait for rx "Ready" Signal
    let mut rx_out = rx.stdout.take().unwrap();
    rx_out.read_exact(&mut [0u8; 1]).expect("Failed to get Sync Signal from rx");

    // 2. Send "GO" Signal to tx, which waits for it
    let tx_in = tx.stdin.as_mut().unwrap();
    tx_in.write(&[1u8]).expect("Failed to send Sync Signal to tx");

    // Wait for both processes to finish
    let _tx_status = tx.wait().unwrap();
    let _rx_status = rx.wait().unwrap();
    
    let pipe_bench_duration = pipe_bench_starttime.elapsed();

    // TODO panic if tx_stats != 0 && rx_status != 0
    //println!("tx exited with: {} \nrx exited with: {}", tx_status.code().unwrap(), rx_status.code().unwrap());
    
    println!("The pipe bench (with execution overhead) took: {:.1} μs", to_microsecs(pipe_bench_duration.as_nanos() as u64));
}
