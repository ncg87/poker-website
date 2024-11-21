/// Schema for the database, creates the database if it doesn't exist
/// 

pub const CREATE_USER_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username VARCHAR(50) UNIQUE NOT NULL,
        password_hash VARCHAR(255) NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

    CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);
    CREATE INDEX IF NOT EXISTS users_email_idx ON users (email);
"#;
// Creates a database and indexes for username and email for faster queries
