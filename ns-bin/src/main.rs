use log::trace;
use ns_log::init;
use std::env::set_var;

fn main() {
    init(None).unwrap();
    set_var("RUST_LOG", "ns-bin=trace");
    log::trace!("Hello, world!");
    log::info!("hello info");
}
