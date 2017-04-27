use chrono;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use diesel::ExpressionMethods;
use serde_json;
use uuid::Uuid;

use db_schema::users;

// TODO:
// * Complete Authentication Infrastructure
// * Create profiles table for personal user information

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    id: i32,
    level_: i32,
    email: String,
    uuid_: Uuid,
    date_created: chrono::DateTime<chrono::UTC>,
    date_modified: chrono::DateTime<chrono::UTC>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub auth_id: i32,
    pub email: String,
    pub uuid_: Uuid,
    pub level_: i32,
}

impl NewUser {
    pub fn new(email: &str, auth: &Auth, level_: i32) -> NewUser {
        let uuid_ = Uuid::new_v4().simple();
        NewUser {
            auth_id: auth.id,
            email: email.into(),
            uuid_: uuid_,
            level_: level_,
        }
    }

    pub fn save(&self, conn: &PgConnection) -> Result<User, DieselError> {
        use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
        use db_schema::users::dsl::*;

        diesel::insert(self)
            .into(users::table)
            .get_result(conn)
            .expect("Error! Failed to save new user.")
    }
}

pub fn get_user_by_email(conn: &PgConnection, get_email: &str) -> Result<User, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db_schema::users::dsl::*;

    users
        .filter(email.eq(get_email))
        .get_result::<User>(conn)
}
