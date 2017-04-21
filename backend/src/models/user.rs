use chrono;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::UTC>,
    modified_on: chrono::DateTime<chrono::UTC>,
}

pub fn get_user_by_email(conn: &PgConnection, get_email: &str) -> Result<User, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db_schema::users::dsl::*;

    users
        .filter(email.eq(get_email))
        .get_result::<User>(conn)
}
