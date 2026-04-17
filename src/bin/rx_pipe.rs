use std::io::{self, Read, Write};
use ipc_benchmark::*;

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = [0u8; BUF_SIZE];
    let mut total_bytes_read: usize = 0;
    let mut timestamp_guard: usize = 0;

    // SYNC: Send "Ready" Signal to main Process
    let mut stdout = io::stdout();
    stdout.write(&[1u8])?;
    stdout.flush()?;

    // Read piped data from stdin / Tx
    loop {
        let bytes_read = stdin.read(&mut buffer[timestamp_guard..])?;
        if bytes_read == 0 { break; }
        total_bytes_read += bytes_read;
        // Ensure that Tx's timestamp is 1. Read properly and 2. Not overwritten
        // (Effectively stdin.read_exact() for the first 8 bytes, but keeping the big buffer)
        if timestamp_guard < 8 { timestamp_guard = 8.min(total_bytes_read); }
    }
    // Data fully received; create timestamp
    let receive_time = get_timestamp_ns();

    if total_bytes_read >= 8 {
        // Tx stored a timestamp in the first 8 bytes of every BUF_SIZEd message-chunk,
        // so extract those and view them as u64.
        let send_time_bytes = &buffer[0..8];
        let send_time = u64::from_le_bytes(send_time_bytes.try_into().unwrap());

        // Send sample to main process
        let elapsed = receive_time - send_time;
        stdout.write_all(&elapsed.to_le_bytes())?;
    }
    // TODO else rumkacken

    Ok(())
}