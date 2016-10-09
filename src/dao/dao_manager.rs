// src/dao/dao_manager.rs

/// DAO Manager
/// Hand out DAOs with database connections

// Import Modules
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::database::DB;
use super::user_dao::UserDAO;
use super::money_map_dao::MoneyMapDAO;
use super::money_map_user_dao::MoneyMapUserDAO;

/// DAO Manager
#[derive(Clone)]
pub struct DAOManager {
    pub db: DB
}

// DAO Manager Methods
impl DAOManager{
    /// Create DAOManager
    ///
    /// # Arguments
    /// db - common::database::DB
    ///
    /// # Returns
    /// `DAOManager`
    pub fn new(db: DB) -> DAOManager{
        DAOManager{
            db: db
        }
    }

    /// Get a UserDAO
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `MMResult<UserDAO>` MMResult wrapping the UserDAO
    pub fn get_user_dao(&self) -> MMResult<UserDAO>{
        match self.db.get_database(){
            Some(db) => Ok(UserDAO::new(db)),
            None => Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }

    /// Get a MoneyMapDAO
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `MMResult<MoneyMapDAO>` MMResult wrapping the MoneyMapDAO
    pub fn get_money_map_dao(&self) -> MMResult<MoneyMapDAO>{
        match self.db.get_database(){
            Some(db) => Ok(MoneyMapDAO::new(db)),
            None => Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }

    /// Get a MoneyMapUserDAO
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `MMResult<MoneyMapUserDAO>` MMResult wrapping the MoneyMapUserDAO
    pub fn get_money_map_user_dao(&self) -> MMResult<MoneyMapUserDAO>{
        match self.db.get_database(){
            Some(db) => Ok(MoneyMapUserDAO::new(db)),
            None => Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }

    //Attempt to create a generic DAO returtning function
    /*pub fn get_DAO(self, daoType: DAOType) -> MMResult<DAO>{
        match self.db.get_database(){
            Some(db) => {
                match daoType{
                    UserDAO => Ok(DAO::UserDAO(UserDAO::new(db)))
                }
            },
            None => return Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }*/
}
