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
use ::models::account_model::{PubAccountModel};

/// Money Map
#[derive(Clone, Debug)]
pub struct MoneyMapModel {
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub users: Option<Vec<MoneyMapUserModel>>,
    pub accounts: Option<Vec<PubAccountModel>>
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct PubMoneyMapModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub users: Option<Vec<MoneyMapUserModel>>,
    pub accounts: Option<Vec<PubAccountModel>>
}

// Money Map Model Methods
impl MoneyMapModel{

    /// Create MoneyMapModel from PubMoneyMapModel
    ///
    /// # Arguments
    /// money_map - PubMoneyMapModel
    ///
    /// # Returns
    /// 'MoneyMapModel'
    pub fn new(money_map: &PubMoneyMapModel) -> MoneyMapModel{
        MoneyMapModel{
            id: match money_map.get_id() { 
                Some(id) => Some(ObjectId::with_string(&id).unwrap()),
                None => None
            },
            name: money_map.get_name(),
            users: money_map.get_users(),
            accounts: money_map.get_accounts()
        }
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
    /// 'Option<Vec<PubAccountModel>>' - name
    pub fn get_accounts(&self) -> Option<Vec<PubAccountModel>>{
        self.accounts.clone()
    }

    /// Set Accounts
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<Vec<PubAccountModel>>' - name
    pub fn set_accounts(&mut self, accounts: Option<Vec<PubAccountModel>>){
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

// Money Map Model Methods
impl PubMoneyMapModel{

    /// Create PubMoneyMapModel from MoneyMapModel
    ///
    /// # Arguments
    /// money_map - MoneyMapModel
    ///
    /// # Returns
    /// 'PubAccountModel'
    pub fn new(mut money_map: MoneyMapModel) -> PubMoneyMapModel{
        PubMoneyMapModel{
            id: match money_map.get_id() { 
                Some(id) => Some(id.to_hex()),
                None => None
            },
            name: money_map.get_name(),
            users: money_map.get_users(),
            accounts: money_map.get_accounts()
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

    /// Get Accounts
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<Vec<PubAccountModel>>' - name
    pub fn get_accounts(&self) -> Option<Vec<PubAccountModel>>{
        self.accounts.clone()
    }

}
