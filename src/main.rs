#![forbid(unsafe_code)]

mod worker;
mod worker_io;
mod worker_task;

fn main() {
    if let Err(error) = worker::run() {
        eprintln!("hat-website-operator-worker: {error}");
        std::process::exit(1);
    }
}
