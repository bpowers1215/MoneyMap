// src/models/account_statement_model.rs

/// Account Statement Model

// Import Modules
// External
use ::bson::oid::ObjectId;
use ::chrono::{Local, TimeZone};
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// Account Statement
#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct AccountStatementModel {
    pub created: Option<i64>
}

// Account Statement Model Methods
impl AccountStatementModel{

    /// Get Created Date
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// Option<i64> Timestamp
    pub fn get_created(&mut self) -> Option<i64>{
        self.created
    }

    /// Set Created Date
    ///
    /// # Arguments
    /// &self
    /// Option<i64> Timestamp
    pub fn set_created(&mut self, timestamp: Option<i64>){
        self.created = timestamp
    }
}
