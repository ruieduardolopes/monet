use slog::{o, Drain, Logger};
use std::fs::{File, OpenOptions};

pub fn init(ingress: bool, interface: String) -> Logger {
    let log_file: File = if ingress {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&format!("ingress-{}.log", interface))
            .unwrap()
    } else {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&format!("egress-{}.log", interface))
            .unwrap()
    };

    let file_decorator = slog_term::PlainDecorator::new(log_file);
    let file_drain = slog_term::FullFormat::new(file_decorator).build().fuse();
    let file_drain = slog_async::Async::new(file_drain).build().fuse();
    slog::Logger::root(file_drain, o!())
}
