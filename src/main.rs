#[macro_use]
extern crate log;
extern crate iron;
extern crate router;
extern crate time;
extern crate simplelog;

pub mod services;

fn main() {
    // start sets up logger, routes and listens on host parameter
    //
    // listen_and_serve(host: &str,  quiet: bool) {
    //   ...
    // }
    let _ = services::server::start("localhost:9000", false);
}
