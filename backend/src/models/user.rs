use chrono;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::UTC>,
    modified_on: chrono::DateTime<chrono::UTC>,
}
