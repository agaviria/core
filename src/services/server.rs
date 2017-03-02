use iron::{Handler, IronResult, Response, Request, Iron};
use iron::prelude::*;
use iron::status;
use simplelog::{Config, TermLogger, LogLevelFilter};

use super::RequestLogger;

struct Ctx {
    msg: String,
}

impl Handler for Ctx {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.msg.clone())))
    }
}

/// listen_and_serve will initialize terminal logger, setup routes and serve.
pub fn start(port: &str, quiet: bool) {
    // Initialize terminal logger
    let _ = TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    // constructor for ping handler
    let echo = Ctx { msg: String::from("Pong!") };

    // declaring router
    let router = router! {
        get_ping: get "/ping" =>  echo,
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
