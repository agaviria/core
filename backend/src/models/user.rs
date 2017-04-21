use chrono;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use uuid::Uuid

#[derive(Debug, Queryable, Serialize)]
#[table_name="auth"]
pub struct Auth {
    pub id: i32,
    pub salt: Vec<u8>,
    pub pwd: Vec<u8>,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}

impl Auth {
   pub fn get_id(conn: &PgConnection, id: &i32) -> Result<Auth, DieselError> {
      use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
      use db_schema::auth::dsl::*;

      let auth_id = id.parse::<i32>().unwrap();
      auth
          .filter(auth::id.eq(auth_id))
          .get_result::<Auth>(conn);
   } 
}

#[derive(Insertable)]
#[table_name="auth"]
pub struct NewAuth {
    pub salt: Vec<u8>,
    pub pwd: Vec<u8>,
}

impl NewAuth {
    pub fn new() {
        unimplemented!(); 
    }

    pub fn create() {
        unimplemented!(); 
    }
}

// TODO: (Authentication Infrastructure)
// * implement NewAuth, NewUser with methods (new and create)
// * Fix mock data to value inserts 
// * Create profiles table for personal user information
// * implement authentication with salt hashing looking at libsodium or Argon2

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
#[table_name="users"]
pub struct NewUser {
    pub auth_id: i32,
    pub email: String,
    pub uuid_: Uuid,
    pub level_: i32,
}

impl NewUser {
    pub fn new() {
        unimplemented!(); 
    }

    pub fn create() {
        unimplemented!(); 
    }
}

pub fn get_user_by_email(conn: &PgConnection, get_email: &str) -> Result<User, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db_schema::users::dsl::*;

    users
        .filter(email.eq(get_email))
        .get_result::<User>(conn)
}


