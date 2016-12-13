// src/models/account_statement_model.rs

/// Account Statement Model

// Import Modules
// External
use ::bson::oid::ObjectId;
use ::chrono::{DateTime, Duration, Local, TimeZone};
use ::chrono::offset::utc::UTC;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// Account Statement
#[derive(Clone)]
pub struct AccountStatementModel {
    pub statement_date: Option<DateTime<UTC>>,
    pub ending_balance: Option<f64>
}

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct OutAccountStatementModel {
    pub statement_date: Option<String>,
    pub ending_balance: Option<f64>
}

// Account Statement Model Methods
impl AccountStatementModel{

    /// Get Statement Date
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// Option<i64> Timestamp
    pub fn get_statement_date(&mut self) -> Option<DateTime<UTC>>{
        self.statement_date
    }

    /// Get Ending Balance
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// Option<f64> Ending Balance
    pub fn get_ending_balance(&mut self) -> Option<f64>{
        self.ending_balance
    }
}

// Out Account Statement Model Methods
impl OutAccountStatementModel{

    /// Create OutAccountStatementModel from AccountStatementModel
    ///
    /// # Arguments
    /// account - AccountStatementModel
    ///
    /// # Returns
    /// 'OutAccountStatementModel'
    pub fn new(mut account: AccountStatementModel) -> OutAccountStatementModel{
        OutAccountStatementModel{
            statement_date:match account.get_statement_date(){
                Some(timestamp) => {
                    Some(timestamp.to_string())
                },
                None => None
            },
            ending_balance: account.get_ending_balance()
        }
    }
}
