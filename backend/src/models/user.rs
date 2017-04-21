use chrono;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use uuid::Uuid

#[derive(Debug, Queryable, Serialize)]
#[table_name="auth"]
pub struct Auth {
    pub id: i32,
    pub salt: Vec<u8>,
    pub password: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::UTC>,
    pub last_updated: chrono::DateTime<chrono::UTC>,
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

// TODO: (Authentication Infrastructure)
// * Add auth table to migrations
// * implement NewUser and create method
// * Create NewAuth with new and create methods
// * Fix user migrations and mock data to include { level_, uuid_, last_updated } field types

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    id: i32,
    level_: i32,
    name: String,
    email: String,
    uuid_: Uuid,
    created_at: chrono::DateTime<chrono::UTC>,
    last_updated: chrono::DateTime<chrono::UTC>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub auth_id: i32,
    pub email: String,
    pub uuid_: Uuid,
    pub level_: i32,
}

pub fn get_user_by_email(conn: &PgConnection, get_email: &str) -> Result<User, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db_schema::users::dsl::*;

    users
        .filter(email.eq(get_email))
        .get_result::<User>(conn)
}


