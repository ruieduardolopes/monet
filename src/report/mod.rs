use crate::capture::results::CaptureResult;

use slog::{info, o, Drain, Logger};
use slog_async::OverflowStrategy;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::path::PathBuf;

pub fn init(ingress: bool, interface: String) -> Logger {
    let home = match env::var_os("HOME") {
        None => {
            panic!("$HOME not set.");
        }
        Some(path) => PathBuf::from(path),
    };
    let log_file: File = if ingress {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&format!("{}/.monet/ingress-{}.log", home, interface))
            .unwrap()
    } else {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&format!("{}/.monet/egress-{}.log", home, interface))
            .unwrap()
    };

    let file_decorator = slog_term::PlainDecorator::new(log_file);
    let file_drain = slog_term::FullFormat::new(file_decorator).build().fuse();
    let file_drain = slog_async::Async::new(file_drain)
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();
    slog::Logger::root(file_drain, o!())
}

pub fn report(reporter: &Logger, records: &Vec<CaptureResult>) -> Result<(), Error> {
    records
        .iter()
        .for_each(|record| info!(reporter, "{}", record));

    Ok(())
}
