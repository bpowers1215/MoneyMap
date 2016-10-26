// src/models/account_model.rs

/// Account Model

// Import Modules
// External
use ::bson::oid::ObjectId;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// Account
#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct AccountModel {
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub account_type: Option<String>,
    pub created: Option<i64>
}

// Account Model Methods
impl AccountModel{

    /// Get ID
    ///
    /// # Arguments
    /// &self
    /// Option<ObjectId>
    pub fn set_id(&mut self, id: Option<ObjectId>){
        self.id = id;
    }

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

    /// Get Name
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - name
    pub fn get_name(&self) -> Option<String>{
        self.name.clone()
    }

    /// Get Account Type
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - type
    pub fn get_account_type(&self) -> Option<String>{
        self.account_type.clone()
    }

    /// Set Created Date
    ///
    /// # Arguments
    /// &self
    /// Option<i64> Timestamp
    pub fn set_created(&mut self, timestamp: Option<i64>){
        self.created = timestamp
    }

    /// Validate Account
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self) -> ValidationResult{

        // Validate
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.name.clone()){
            validation_result.add_error("name".to_string(), "Name is required.".to_string());
        }
        if !Validators::not_empty_string(self.account_type.clone()){
            validation_result.add_error("account_type".to_string(), "Account Type is required.".to_string());
        }

        validation_result
    }//end validate
}
