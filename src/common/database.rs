// src/common/database.rs

/// Database Structure
/// Maintain database connection

//Import Crates
extern crate bson;
extern crate mongodb;

//Import Modules
use self::mongodb::{Client, ThreadedClient};
use self::mongodb::db::ThreadedDatabase;
use self::mongodb::error::Result as MongoResult;
use super::mm_result::{MMResult, MMError, MMErrorKind};
use super::config::Config;

//Constants
//Errors
static ERROR_DB_MISS: &'static str = "Error: No database connection";

/// Represent a Database Connection
#[derive(Clone)]
pub struct DB{
    db_host: String,
    db_port: u16,
    db_name: String,
    db_user: String,
    db_pass: String,
    database: Option<mongodb::db::Database>
}

impl DB{

    /// Create A DB struct and establish a connection to the database
    ///
    /// # Returns
    /// `MMResult<DB>` - DB struct
    pub fn new(config: Config) -> MMResult<DB>{
        let db_host = match config.database.host{
            Some(v) => v,
            None => {
                warn!("Database configuration missing: host");
                return Err(MMError::new("Database configuration missing: host".to_string(), MMErrorKind::Database));
            }
        };
        let db_port = match config.database.port{
            Some(v) => v,
            None => {
                warn!("Database configuration missing: port");
                return Err(MMError::new("Database configuration missing: port".to_string(), MMErrorKind::Database));
            }
        };
        let db_name = match config.database.name{
            Some(v) => v,
            None => {
                warn!("Database configuration missing: name");
                return Err(MMError::new("Database configuration missing: name".to_string(), MMErrorKind::Database));
            }
        };
        let db_user = match config.database.username{
            Some(v) => v,
            None => {
                warn!("Database configuration missing: username");
                return Err(MMError::new("Database configuration missing: username".to_string(), MMErrorKind::Database));
            }
        };
        let db_pass = match config.database.password{
            Some(v) => v,
            None => {
                warn!("Database configuration missing: password");
                return Err(MMError::new("Database configuration missing: password".to_string(), MMErrorKind::Database));
            }
        };
        let mut db = DB {
            db_host: db_host,
            db_port: db_port as u16,
            db_name: db_name,
            db_user: db_user,
            db_pass: db_pass,
            database: None
        };
        db.database = match db.initialize_db_connection(){
            Ok(database) => Some(database),
            Err(_) => None
        };
        match db.authenticate(){
            Err(e) => {
                warn!("{}", e);
            },
            _ => {}
        }
        Ok(db)
    }

    /// Get db_host
    ///
    /// # Returns
    /// `String` - Database Host
    pub fn db_host(self) -> String{
        self.db_host
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
    /// `String` - Database name
    pub fn db_name(self) -> String{
        self.db_name
    }

    /// Get db_user
    ///
    /// # Returns
    /// `String` - Database user name
    pub fn db_user(self) -> String{
        self.db_user
    }

    ///Establish a database connection and store the connection in the DB struct
    ///
    /// # Arguments
    ///
    /// * `self` - self DB struct
    ///
    /// # Returns
    /// `MMResult<self::mongodb::db::Database>` - Mongo DB
    fn initialize_db_connection(&self) -> MMResult<mongodb::db::Database>{
        let mut client_wrapper = None;

        // Connect to the mongo db instance
        match Client::connect(&self.db_host, self.db_port){
            Ok(db_client) => {
                client_wrapper = Some(db_client);
            },
            Err(_) => {}
        }

        match client_wrapper{
            Some(client) => {
                // Get database
                Ok(client.db(&self.db_name))
            },
            None => {
                //Database connection could not be established
                warn!("Error establishing database connection");
                return Err(MMError::new("Error establishing database connection.".to_string(), MMErrorKind::Database));
            }
        }
    }

    ///Authenticate DB user
    ///
    /// # Arguments
    /// * `self` - self DB struct
    ///
    /// # Returns
    /// `MMResult<()>`
    fn authenticate(&self) -> MMResult<()>{
        match &self.database{
            &Some(ref db) => {
                // Authenticate
                match db.auth(&self.db_user, &self.db_pass){
                    Ok(_) => Ok(()),
                    Err(_) => {
                        Err(MMError::new("Failed to authorize database user.".to_string(), MMErrorKind::Database))
                    }
                }
            },
            &None => {
                Err(MMError::new("No database connection to authenticate on.".to_string(), MMErrorKind::Database))
            }
        }

    }

    /// Get a database client
    ///
    /// # Arguments
    /// * `self` - self DB struct
    ///
    /// # Returns
    /// `Option<mongodb::db::Database>` Cloned database client
    pub fn get_database(&self) -> Option<mongodb::db::Database>{
        match self.database{
            Some(ref database) => Some(database.clone()),
            None => None
        }
    }

    pub fn get_count(&self) -> MMResult<i64>{
        match self.get_database(){
            Some(ref database) => {
                let coll = database.collection("users");
                match coll.count(None, None){
                    Ok(count) => Ok(count),
                    Err(e) => {
                        warn!("Get Count: {}", e);
                        Err(MMError::new("Error getting count from DB".to_string(), MMErrorKind::Database))
                    }
                }
            },
            None => {
                warn!("{}", ERROR_DB_MISS);
                Err(MMError::new(ERROR_DB_MISS.to_string(), MMErrorKind::Database))
            }
        }
    }
}
