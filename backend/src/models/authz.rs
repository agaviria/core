use chrono;
use diesel::prelude::*;
use diesel::{ExpressionMethods, LoadDsl, FilterDsl};
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;
use uuid::Uuid;
use serde_json;

use db_schema::auth;
use crypto;

#[table_name = "auth"]
#[derive(Debug, Queryable, Serialize)]
pub struct Auth {
    pub id: i32,
    pub salt: Vec<u8>,
    pub pwd: Vec<u8>,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}

impl Auth {
    pub fn get_id(conn: &PgConnection, id: &i32) -> Result<Auth, DieselError> {
        use db_schema::auth::dsl::*;

        let auth_id = id.parse::<i32>().unwrap();
        auth.filter(auth::id.eq(auth_id))
            .get_result::<Auth>(conn);
    }
}

#[table_name = "auth"]
#[derive(Insertable)]
pub struct NewAuth {
    pub salt: Vec<u8>,
    pub pwd: Vec<u8>,
}

impl NewAuth {
    pub fn new(pwd_string: &str) -> NewAuth {
        let salt = crypto::generate_salt().expect("salt generate failed");
        let hashed_password: Vec<u8> = crypto::hash(pwd_string, salt).expect("hash method failed");

        NewAuth {
            salt: salt,
            pwd: hashed_password,
        }
    }

    pub fn save(&self, conn: &PgConnection) -> Result<Auth, DieselError> {
        use diesel::insert;
        use db_schema::auth::dsl::*;

        insert(self)
            .into(auth::table)
            .get_result(conn)
            .expect("Error! Failed to save credentials into auth table.")
    }
}
