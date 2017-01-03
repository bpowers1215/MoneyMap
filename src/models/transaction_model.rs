// src/models/transaction_model.rs

/// Transaction Model

// Import Modules
// External
use ::bson::oid::ObjectId;
use ::chrono::{DateTime, Duration, Local, TimeZone};
use ::chrono::offset::utc::UTC;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// Transaction
#[derive(Clone)]
pub struct TransactionModel {
    pub id: Option<ObjectId>,
    pub money_map_id: Option<ObjectId>,
    pub account_id: Option<ObjectId>,
    pub datetime: Option<DateTime<UTC>>,
    pub payee: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<ObjectId>,
    pub amount: Option<f64>,
    pub transaction_type: Option<String>,
    pub status: Option<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PubTransactionModel {
    pub id: Option<ObjectId>,
    pub money_map_id: Option<ObjectId>,
    pub account_id: Option<ObjectId>,
    pub datetime: Option<String>,
    pub payee: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<ObjectId>,
    pub amount: Option<f64>,
    pub transaction_type: Option<String>,
    pub status: Option<String>
}

// Transaction Model Methods
impl TransactionModel{

    /// Get ID
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<ObjectId>' - id
    pub fn get_id(&self) -> Option<ObjectId>{
        self.id.clone()
    }

    /// Set ID
    ///
    /// # Arguments
    /// &self
    pub fn set_id(&mut self, id: ObjectId) {
        self.id = Some(id);
    }

    /// Get Payee
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - Entity
    pub fn get_payee(&self) -> Option<String>{
        self.payee.clone()
    }

    /// Get Description
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - description
    pub fn get_description(&self) -> Option<String>{
        self.description.clone()
    }

    /// Validate Transaction
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self) -> ValidationResult{

        //validate Transaction
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.payee.clone()){
            validation_result.add_error("payee".to_string(), "Payee is required.".to_string());
        }

        validation_result
    }//end validate
}
