// src/dao/dao_manager.rs

/// DAO Manager
/// Hand out DAOs with database connections

extern crate mongodb;

// Import Modules
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::database::DB;
use super::account_dao::AccountDAO;
use super::account_statement_dao::AccountStatementDAO;
use super::user_dao::UserDAO;
use super::money_map_dao::MoneyMapDAO;
use super::money_map_user_dao::MoneyMapUserDAO;
use super::transaction_dao::TransactionDAO;

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
    pub fn get_database(&self) -> Option<mongodb::db::Database>{
        self.db.clone().get_database()
    }

    /// Get a UserDAO
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `MMResult<UserDAO>` MMResult wrapping the UserDAO
    pub fn get_user_dao(&self) -> MMResult<UserDAO>{
        match self.db.clone().get_database(){
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
        match self.db.clone().get_database(){
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
        match self.db.clone().get_database(){
            Some(db) => Ok(MoneyMapUserDAO::new(db)),
            None => Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }

    /// Get a AccountDAO
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `MMResult<AccountDAO>` MMResult wrapping the AccountDAO
    pub fn get_account_dao(&self) -> MMResult<AccountDAO>{
        match self.db.clone().get_database(){
            Some(db) => Ok(AccountDAO::new(db)),
            None => Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }

    /// Get a AccountStatementDAO
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `MMResult<AccountStatementDAO>` MMResult wrapping the AccountStatementDAO
    pub fn get_account_statement_dao(&self) -> MMResult<AccountStatementDAO>{
        match self.db.clone().get_database(){
            Some(db) => Ok(AccountStatementDAO::new(db)),
            None => Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }

    /// Get a TransactionDAO
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `MMResult<TransactionDAO>` MMResult wrapping the TransactionDAO
    pub fn get_transaction_dao(&self) -> MMResult<TransactionDAO>{
        match self.db.clone().get_database(){
            Some(db) => Ok(TransactionDAO::new(db)),
            None => Err(MMError::new("Error: database connection not established", MMErrorKind::Database))
        }
    }
}
