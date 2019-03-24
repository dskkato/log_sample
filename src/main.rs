#[macro_use]
extern crate log;
extern crate env_logger as logger;

use log::Level;

fn main() {
    logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
