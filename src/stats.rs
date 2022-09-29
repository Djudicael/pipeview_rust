use std::io::Result;

use std::sync::mpsc::{Receiver, Sender};

pub fn stats_loop(
    silent: bool,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;

    loop {
        // receive the byte from the read thread
        let buffer: Vec<u8> = stats_rx.recv().unwrap();
        let numb_bytes = buffer.len();
        total_bytes += numb_bytes;
        if !silent {
            eprint!("\r{}", total_bytes);
        }

        //send vector to write loop
        if write_tx.send(buffer).is_err() {
            break;
        }

        if numb_bytes == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }

    Ok(())
}