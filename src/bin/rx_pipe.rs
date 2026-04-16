use std::io::{self, Read};
use ipc_benchmark::{get_timestamp_ns, to_microsecs, BUF_SIZE_BYTES};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = [0u8; BUF_SIZE_BYTES];

    // Read piped data from stdin / tx
    let mut total_bytes_read: usize = 0;
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
        // (Endianness can be safely assumed as Little for Linux & Windows)
        let send_time_bytes = &buffer[0..8];
        let send_time = u64::from_le_bytes(send_time_bytes.try_into().unwrap());

        // Sanity checks:
        println!("Send time was {send_time} \nReceive time was {receive_time}");
        println!("Rx received {} bytes from stdin", total_bytes_read);

        // TODO log bench data
        let elapsed_ns = receive_time - send_time;
        println!("Sending -> Receiving took {:.1} μs", to_microsecs(elapsed_ns));
    }
    // TODO else rumkacken

    Ok(())
}