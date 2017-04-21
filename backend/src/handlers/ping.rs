//! ping test handler
use iron::status;
use iron::prelude::*;

use services::storage::DbPooledConnection;

// ping handler is simply just to  test if core service is available.
pub fn ping(_: &mut Request, _: DbPooledConnection) -> IronResult<Response> {
    Ok(Response::with((status::Ok, format!("Pong!"))))
}
