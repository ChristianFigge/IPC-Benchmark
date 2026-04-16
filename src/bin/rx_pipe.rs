use std::io::{self, Read, Write};
use ipc_benchmark::{get_timestamp_ns, to_microsecs, BUF_SIZE_BYTES};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = [0u8; BUF_SIZE_BYTES];
    let mut total_bytes_read: usize = 0;

    // SYNC: Send "Ready" Signal to main Process
    let mut stdout = io::stdout();
    stdout.write(&[1u8])?;
    stdout.flush()?;

    // Read piped data from stdin / tx
    loop {
        let bytes_read = stdin.read(&mut buffer)?;
        if bytes_read == 0 { break; }
        total_bytes_read += bytes_read;
    }
    // Data fully received; create timestamp
    let receive_time = get_timestamp_ns();

    if total_bytes_read >= 8 {
        // Tx timestamped the first 8 bytes of every message-chunk,
        // so extract those bytes and view them as u64.
        let send_time_bytes = &buffer[0..8];
        let send_time = u64::from_le_bytes(send_time_bytes.try_into().unwrap());

        // Sanity checks:
        eprintln!("Send time was {send_time} \nReceive time was {receive_time}");
        eprintln!("Rx received {total_bytes_read} bytes from stdin");

        // TODO log bench data
        let elapsed_micros = to_microsecs(receive_time - send_time);
        eprintln!("Sending -> Receiving took {elapsed_micros:.1} μs");
    }
    // TODO else rumkacken

    Ok(())
}