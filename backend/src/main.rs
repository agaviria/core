#[macro_use]
extern crate log;
#[macro_use]
extern crate router;
extern crate iron;
#[macro_use]
extern crate mime;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate mount;
extern crate time;
extern crate chrono;
extern crate simplelog;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod services;
pub mod handlers;
pub mod models;
pub mod db_schema;

fn main() {
    // start sets up logger, routes and listens on host parameter
    // host will try to connect to proxy request event in port :8080
    //
    // start(host: &str,  quiet: bool) {
    //   ...
    // }
    let _ = services::server::start("127.0.0.1:8081", false);
}
