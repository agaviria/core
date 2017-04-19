//! ping test handler
use iron::status;
use iron::prelude::*;

use services::storage::DbPooledConnection;

pub fn ping(_: &mut Request, _: DbPooledConnection) -> IronResult<Response> {
    Ok(Response::with((status::Ok, format!("Pong!"))))
}
