//! ping test handler
use iron::{Handler, Request, Response, IronResult};
use iron::status;

pub struct Ping;

impl Handler for Ping {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, format!("Pong!"))))
    }
}
