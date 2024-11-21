/// Database module for the server

use tokio_postgres::{Client, NoTls};
use crate::db::schema::CREATE_USER_TABLE;

pub struct Database {
    pub client: Client, // Postgres client to interact with the database
}

impl Database {
    // Create a new database instance
    pub async fn new(connection_string: &str) -> Result<Self, tokio_postgres::Error> {

        // Connect to the database
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        // Spawn a task (async thread) to run the connection in the background
        tokio::spawn(async move { // Needs to take ownership of connection
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });
        // Initialize shape of the database, and runs all commands in constant
        client.batch_execute(CREATE_USER_TABLE).await?;

        // Return the database instance
        Ok(Database { client })
    }

    // Gets the client
    pub fn get_client(&self) -> &Client {
        &self.client
    }
}

// Going to implement actual TLS in the future here, with security, certificates, etc.

