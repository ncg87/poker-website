/// User struct for database
/// 

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use chrono::{DateTime, Utc};

// User struct for database
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64, // For database
    pub email: String,
    pub username: String, // For display and user
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Allows for conversion of a postgres database row into a User struct
impl From<Row> for User {
    fn from(row: Row) -> Self {
        User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password: row.get("password"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
