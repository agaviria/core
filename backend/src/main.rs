#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate iron;
#[macro_use]
extern crate router;
#[macro_use]
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate simplelog;

pub mod services;
pub mod handlers;

fn main() {
    // start sets up logger, routes and listens on host parameter
    // host will try to connect to proxy request event in port :8080
    //
    // listen_and_serve(host: &str,  quiet: bool) {
    //   ...
    // }
    let _ = services::server::start("127.0.0.1:8081", false);
}
