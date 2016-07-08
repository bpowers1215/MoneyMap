// src/dao/dao_controller.rs

/// DAO Controller
/// Hand out DAOs with database connections

//Import Modules
use ::common::config::Config;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::database::DB;
use super::user_dao::UserDAO;
use super::money_map_dao::MoneyMapDAO;

/*pub enum DAOType{
    UserDAO
}
pub enum DAO{
    UserDAO(UserDAO)
}*/

/// DAO Controller
pub struct DAOController {
    pub db: DB
}

// DAO Controller Methods
impl DAOController{
    /// Create DAOController
    ///
    /// # Arguments
    /// db - common::database::DB
    ///
    /// # Returns
    /// `DAOController`
    pub fn new(db: DB) -> DAOController{
        DAOController{
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
            None => Err(MMError::new("Error: database connection not established".to_string(), MMErrorKind::Database))
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
            None => Err(MMError::new("Error: database connection not established".to_string(), MMErrorKind::Database))
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
            None => return Err(MMError::new("Error: database connection not established".to_string(), MMErrorKind::Database))
        }
    }*/
}
