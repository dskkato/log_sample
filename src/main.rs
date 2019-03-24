#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use log::Level;

fn main() {
    // logger::init().unwrap();
    logger::init_with_level(Level::Info).unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
