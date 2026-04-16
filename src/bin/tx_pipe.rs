use ipc_benchmark::{BUF_SIZE_BYTES, get_timestamp_ns};
use std::env;
use std::io::{self, Write, Error, ErrorKind};

const BUF_SIZE_U64: usize = BUF_SIZE_BYTES / size_of::<u64>();

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

    // Create data buffer; u64 is needed for the timestamp
    let mut arr = [0u64; BUF_SIZE_U64]; //[get_timestamp_ns(); BUF_SIZE_U64];

    // Get a *u8 view on an u64 array for stdout.write_all()
    // Endian-safe alternative with a bit more overhead: timestamp.to_le_bytes() + for i in 0..8
    // or use the bytemuck crate
    let buffer: &[u8] =
        unsafe { std::slice::from_raw_parts(arr.as_ptr() as *const u8, BUF_SIZE_BYTES) };

    // Use stderr for status prints, so it doesnt get into the pipe
    eprintln!(
        "Tx is about to write {} bytes to stdout",
        remaining_msg_size
    );

    // Stamp the current time into the first 8 bytes
    #[allow(unused_assignments)] // Avoid Compiler confusion (it thinks arr is unused)
    { arr[0] = get_timestamp_ns(); }

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
