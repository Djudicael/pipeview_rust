use std::io::Result;

use crossbeam::channel::Receiver;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;

    loop {
        // receive the byte from the read thread

        let numb_bytes = stats_rx.recv().unwrap();
        total_bytes += numb_bytes;
        if !silent {
            eprint!("\r{}", total_bytes);
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
