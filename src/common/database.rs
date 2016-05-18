// src/common/database.rs

/// Database Structure
/// Maintain database connections and transactions

//Import Crates
extern crate bson;
extern crate mongodb;

//Import Modules
use self::mongodb::{Client, ThreadedClient};
use self::mongodb::db::ThreadedDatabase;
use self::mongodb::error::Result as MongoResult;

//Define Constants
const MONGO_DB_PORT: u16 = 27017;
static MONGO_DB_HOST: &'static str = "mongoDB";
static MONGO_DB_NAME: &'static str = "todo";
static MONGO_DB_USER: &'static str = "todo";
static MONGO_DB_PW: &'static str = "SECRET_PASSWORD";

/// Represent a Database Connection
pub struct DB{
    db_host: &'static str,
    db_port: u16,
    db_name: &'static str,
    db_user: &'static str,
    db_pass: &'static str,
    db: Option<mongodb::db::Database>
}

impl DB{
    /// Returns a database connection
    pub fn new() -> DB{
        let mut db = DB {
            db_host: MONGO_DB_HOST,
            db_port: MONGO_DB_PORT,
            db_name: MONGO_DB_NAME,
            db_user: MONGO_DB_USER,
            db_pass: MONGO_DB_PW,
            db: None
        };
        db.get_db_connection();
        db
    }
    /// Get db_host
    pub fn db_host(&self) -> &'static str{
        &self.db_host
    }
    /// Get db_port
    pub fn db_port(self) -> u16{
        self.db_port
    }
    /// Get db_name
    pub fn db_name(&self) -> &'static str{
        &self.db_name
    }
    /// Get db_user
    pub fn db_user(&self) -> &'static str{
        &self.db_user
    }
    fn get_db_connection(&mut self){
        // Connect to the database
        let client = Client::connect(&self.db_host, self.db_port)
            .ok().expect("Error establishing database connection.");

        // Get admin DB
        let db = client.db(MONGO_DB_NAME);

        // Authenticate admin user for admin DB
        db.auth(&self.db_user, &self.db_pass)
            .ok().expect("Failed to authorize user 'todo'.");

        self.db = Some(db);
    }
}
