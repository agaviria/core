pub mod ping;

use diesel::prelude::*;
use router::Router;

use serde_json;
use iron::{Response, Request, IronResult};
use iron::status;

use services::storage::DbPooledConnection;
use services::response;
use models::user::*;
use db_schema::users;

use std::error::Error;

// GET user/:email
pub fn get_user(req: &mut Request, conn: DbPooledConnection) -> IronResult<Response> {
    let ref user_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("email")
        .unwrap_or("/");
    let user_email = user_id.to_string();

    // check if the request executed with success
    match get_user_by_email(&*conn, &user_email) {
        Ok(u) => response::ok(serde_json::to_string(&u).unwrap()),
        Err(e) => {
            response::bad_request(format!("email does not exist in database {}", e.description()))
        }
    }
}
