#![feature(custom_attribute)]

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
extern crate uuid;
extern crate time;
extern crate chrono;
extern crate simplelog;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate dotenv;
pub mod crypto;
pub mod db_schema;
pub mod handlers;
pub mod models;
pub mod services;

fn main() {
    // start sets up logger, routes and listens on host parameter
    // host will try to connect to proxy request event in port :8080
    //
    // start(host: &str,  quiet: bool) {
    //   ...
    // }
    let _ = services::server::start("127.0.0.1:8081", false);
}
