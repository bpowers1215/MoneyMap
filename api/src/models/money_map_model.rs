// src/models/money_map_model.rs

/// Money Map Model

// Import Modules
// External
use ::bson::oid::ObjectId;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};
// Models
use ::models::money_map_user_model::{MoneyMapUserModel};
use ::models::account_model::{OutAccountModel};

/// Money Map
#[derive(RustcDecodable, RustcEncodable)]
pub struct MoneyMapModel {
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub users: Option<Vec<MoneyMapUserModel>>,
    pub accounts: Option<Vec<OutAccountModel>>
}

// Money Map Model Methods
impl MoneyMapModel{

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

    /// Get Users
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<Vec<MoneyMapUserModel>>' - name
    pub fn get_users(&self) -> Option<Vec<MoneyMapUserModel>>{
        self.users.clone()
    }

    /// Set Users
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<Vec<MoneyMapUserModel>>' - name
    pub fn set_users(&mut self, users: Option<Vec<MoneyMapUserModel>>){
        self.users = users;
    }

    /// Get Accounts
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<Vec<OutAccountModel>>' - name
    pub fn get_accounts(&self) -> Option<Vec<OutAccountModel>>{
        self.accounts.clone()
    }

    /// Set Accounts
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<Vec<OutAccountModel>>' - name
    pub fn set_accounts(&mut self, accounts: Option<Vec<OutAccountModel>>){
        self.accounts = accounts;
    }

    /// Validate Money Map
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self) -> ValidationResult{

        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.name.clone()){
            validation_result.add_error("name".to_string(), "Name is required.".to_string());
        }

        validation_result
    }//end validate
}
