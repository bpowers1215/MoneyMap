// src/models/account_model.rs

/// Account Model

// Import Modules
// External
use ::bson::oid::ObjectId;
use ::chrono::{DateTime, Local, TimeZone};
use ::std::error::Error;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// Account
#[derive(Clone)]
pub struct AccountModel {
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub account_type: Option<String>,
    pub created: Option<i64>
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct PubAccountModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub account_type: Option<String>,
    pub created: Option<String>
}

// Account Model Methods
impl AccountModel{

    /// Create AccountModel from PubAccountModel
    ///
    /// # Arguments
    /// account - PubAccountModel
    ///
    /// # Returns
    /// 'AccountModel'
    pub fn new(mut account: PubAccountModel) -> AccountModel{
        AccountModel{
            id: match account.get_id() { 
                Some(id) => Some(ObjectId::with_string(&id).unwrap()),
                None => None
            },
            name: account.get_name(),
            account_type: account.get_account_type(),
            created:match account.get_created(){
                Some(created_date) => {
                    match DateTime::parse_from_rfc2822(&created_date){
                        Ok(cd) => Some(cd.timestamp()),
                        Err(e) => {
                            error!("{}",e.description());
                            None
                        }
                    }
                },
                None => None
            }
        }
    }

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

    /// Validate new Account
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate_new(&self) -> ValidationResult{

        // Validate
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.name.clone()){
            validation_result.add_error("name".to_string(), "Name is required.".to_string());
        }
        if !Validators::not_empty_string(self.account_type.clone()){
            validation_result.add_error("account_type".to_string(), "Account Type is required.".to_string());
        }

        validation_result
    }//end validate_new

    /// Validate Existing Account
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate_existing(&self) -> ValidationResult{

        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::empty(&self.name){
            if !Validators::not_empty_string(self.name.clone()){
                validation_result.add_error("name".to_string(), "Name is required.".to_string());
            }
        }
        if !Validators::empty(&self.account_type){
            if !Validators::not_empty_string(self.account_type.clone()){
                validation_result.add_error("account_type".to_string(), "Account Type is required.".to_string());
            }
        }
        validation_result
    }
}


// Out Account Model Methods
impl PubAccountModel{

    /// Create PubAccountModel from AccountModel
    ///
    /// # Arguments
    /// account - AccountModel
    ///
    /// # Returns
    /// 'PubAccountModel'
    pub fn new(mut account: AccountModel) -> PubAccountModel{
        PubAccountModel{
            id: match account.get_id() { 
                Some(id) => Some(id.to_hex()),
                None => None
            },
            name: account.get_name(),
            account_type: account.get_account_type(),
            created:match account.get_created(){
                Some(timestamp) => {
                    Some(Local.timestamp(timestamp.clone(), 0).to_rfc2822())
                },
                None => None
            }
        }
    }

    /// Get ID
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - id
    pub fn get_id(&self) -> Option<String>{
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

    /// Get Created Date
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// Option<String> Timestamp
    pub fn get_created(&mut self) -> Option<String>{
        self.created.clone()
    }
}
