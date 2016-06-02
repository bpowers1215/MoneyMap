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
use super::mm_result::{MMResult, MMError, MMErrorKind};

//Connection Settings
const MONGO_DB_PORT: u16 = 27017;
static MONGO_DB_HOST: &'static str = "money-map-db";
static MONGO_DB_NAME: &'static str = "moneyMap";
static MONGO_DB_USER: &'static str = "money_map_client";
static MONGO_DB_PW: &'static str = "ds(9sj@^DFe>D;3kc";
//Constants
//Errors
static ERROR_DB_MISS: &'static str = "Error: No database connection";

/// Represent a Database Connection
pub struct DB{
    db_host: &'static str,
    db_port: u16,
    db_name: &'static str,
    db_user: &'static str,
    db_pass: &'static str,
    database: Option<mongodb::db::Database>
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
    ///
    /// * `self` - self DB struct
    ///
    /// # Returns
    /// `MMResult<()>`
    fn authenticate(&self) -> MMResult<()>{
        match &self.database{
            &Some(ref db) => {
                // Authenticate
                match db.auth(self.db_user, self.db_pass){
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

    pub fn get_count(&self) -> MMResult<i64>{
        match self.database{
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
