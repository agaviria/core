pub mod ping;

use diesel::prelude::*;
use router::Router;

use serde_json;
use iron::{Response, Request, IronResult};
use iron::status;

use services::storage::DbPooledConnection;
use models::user::User;
use db_schema::users;

pub fn get_user(req: &mut Request, conn: DbPooledConnection) -> IronResult<Response> {
    let ref user_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap_or("/");
    let user_id = user_id.parse::<i32>().unwrap();

    let user_data = users::table
        .filter(users::id.eq(user_id))
        .load::<User>(&*conn);

    let ref user_data = &user_data.unwrap()[0];

    let response_data = serde_json::to_string(user_data).unwrap();

    Ok(Response::with((status::Ok, response_data)))
}
