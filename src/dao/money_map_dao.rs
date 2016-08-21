// src/dao/money_map_dao.rs

/// Money Map DAO
/// Handle all database interaction for Money Map collection

//import
extern crate mongodb;

//Import Modules
use ::mongodb::{Client, ThreadedClient};
use ::mongodb::db::ThreadedDatabase;
use ::mongodb::error::Result as MongoResult;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};

/// Money Map DAO
pub struct MoneyMapDAO{
    db: mongodb::db::Database
}

// Money Map DAO Methods
impl MoneyMapDAO{
    /// Create MoneyMapDAO
    ///
    /// # Arguments
    /// db - mongodb::db::Database Cloned database connection
    ///
    /// # Returns
    /// `MoneyMapDAO`
    pub fn new(db: mongodb::db::Database) -> MoneyMapDAO{
        MoneyMapDAO{
            db: db
        }
    }
    pub fn test(self){
        info!("We have a MoneyMapDAO!!!");
    }
}
