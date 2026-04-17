use ipc_benchmark::{BUF_SIZE, get_timestamp_ns, args_error};
use std::env;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    // Get message size from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return args_error(format!("Usage: {} <num_bytes>", args[0]));
    }
    // Parse arg string to usize and check validity of value
    let mut remaining_msg_size: usize =
        match args[1].parse() {
            Err(_) => return args_error("failed to parse msg_size".to_string()),
            Ok(n) =>
                // We need a message of at least 8 Bytes to include the timestamp
                if n < 8 { return args_error("msg_size must be at least 8".to_string()); }
                else { n }
        };
    // Create data buffer
    let mut buffer = [42u8; BUF_SIZE];

    // SYNC: Wait for "Go" Signal from main process
    io::stdin().read_exact(&mut [0u8; 1])?;
    
    // Stamp the current time into the first 8 bytes
    buffer[0..8].copy_from_slice(&get_timestamp_ns().to_le_bytes());

    // Transmit data over pipe
    let mut stdout = io::stdout();
    while remaining_msg_size > 0 {
        let to_write = remaining_msg_size.min(BUF_SIZE);
        stdout.write_all(&buffer[..to_write])?;
        remaining_msg_size -= to_write;
    }
    
    Ok(())
}
