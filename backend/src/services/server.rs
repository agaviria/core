use iron::Iron;
use iron::prelude::*;
use simplelog::{Config, TermLogger, LogLevelFilter};

use super::RequestLogger;
pub use handlers;

/// listen_and_serve will initialize terminal logger, setup routes and serve.
pub fn start(port: &str, quiet: bool) {
    // Initialize terminal logger
    let _ = TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    // constructor for ping handler
    // let echo = Ctx { msg: String::from("Pong!") };

    // declaring router
    let router = router! {
        get_ping: get "/ping" => handlers::ping::Ping,
        get_many_events: get "/events" => handlers::handle_get_many_events,
        post_new_event: post "/events" => handlers::handle_post_new_event,
    };

    // chain methods [log]
    let mut chain = Chain::new(router);
    if !quiet {
        chain.link_before(RequestLogger);
        chain.link_after(RequestLogger);
    }
    // initiate and serve Iron server
    match Iron::new(chain).http(port) {
        Ok(http) => info!("Listening and serving...{:?}", http),
        Result::Err(err) => panic!("Error starting Iron server: {}", err),
    }
}
