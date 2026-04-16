use ipc_benchmark::{BUF_SIZE_BYTES, get_timestamp_ns};
use std::env;
use std::io::{self, Write, Error, ErrorKind};

fn main() -> io::Result<()> {
    // Get size of message to transmit from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <num_bytes>", args[0]);
        std::process::exit(1);
    }

    // Parse arg string to usize and check validity of value
    // (We need a message of at least 8 Bytes to include the timestamp)
    let mut remaining_msg_size: usize = match args[1].parse() {
        Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "failed to parse num_bytes")),
        Ok(n) => {
            if n < 8 {
                return Err(Error::new(ErrorKind::InvalidInput,"num_bytes must be at least 8"));
            } else { n }
        }
    };

    // Status print using stderr, so it doesnt get into the pipe
    eprintln!("Tx is about to write {remaining_msg_size} bytes to stdout");

    // Create data buffer & Stamp the current time into the first 8 bytes
    let mut buffer = [42u8; BUF_SIZE_BYTES];
    buffer[0..8].copy_from_slice(&get_timestamp_ns().to_le_bytes());

    // TRANSMIT DATA OVER PIPE
    // Get a stdout handle
    let mut stdout = io::stdout();

    // Stream data to stdout into the pipe
    while remaining_msg_size > 0 {
        let to_write = remaining_msg_size.min(BUF_SIZE_BYTES);
        stdout.write_all(&buffer[..to_write])?;
        remaining_msg_size -= to_write;
    }

    Ok(())
}
