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
static MONGO_DB_HOST: &'static str = "money-map-db";
static MONGO_DB_NAME: &'static str = "moneyMap";
static MONGO_DB_USER: &'static str = "money_map_client";
static MONGO_DB_PW: &'static str = "ds(9sj@^DFe>D;3kc";

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

    /// Create A DB struct and establish a connection to the database
    ///
    /// # Returns
    /// `DB` - DB struct
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
    ///
    /// # Returns
    /// `&'static str` - Database Host
    pub fn db_host(&self) -> &'static str{
        &self.db_host
    }

    /// Get db_port
    ///
    /// # Returns
    /// `u16` - Database port
    pub fn db_port(self) -> u16{
        self.db_port
    }

    /// Get db_name
    ///
    /// # Returns
    /// `&'static str` - Database name
    pub fn db_name(&self) -> &'static str{
        &self.db_name
    }

    /// Get db_user
    ///
    /// # Returns
    /// `&'static str` - Database user name
    pub fn db_user(&self) -> &'static str{
        &self.db_user
    }

    ///Establish a database connection and store the connection in the DB struct
    ///
    /// # Arguments
    ///
    /// * `self` - self DB struct
    fn get_db_connection(&mut self){
        // Connect to the mongo db instance
        let client = Client::connect(&self.db_host, self.db_port)
            .ok().expect("Error establishing database connection.");

        // Get database
        let db = client.db(&self.db_name);

        // Authenticate
        db.auth(&self.db_user, &self.db_pass)
            .ok().expect("Failed to authorize user 'todo'.");

        self.db = Some(db);
    }
}
