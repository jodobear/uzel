#![forbid(unsafe_code)]

use std::io::{self, Write};

fn main() {
    println!("UZEL_NAPD_READY role={}", napd::PROCESS_ROLE);
    io::stdout().flush().expect("flush daemon readiness");

    if std::env::args().any(|argument| argument == "--check") {
        return;
    }

    loop {
        std::thread::park();
    }
}
