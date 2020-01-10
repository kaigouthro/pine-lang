mod pine_server;
mod stdio_server;

#[macro_use]
extern crate log;
use env_logger;

fn main() {
    env_logger::init();
    info!("Starting language server");
    stdio_server::start();
}
