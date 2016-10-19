// src/models/money_map_user_model.rs

/// Money Map User Model

// Import Modules
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};
// Models
use ::models::user_model::{OutUserModel};

/// Money Map User
#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct MoneyMapUserModel {
    pub user: Option<OutUserModel>,
    pub owner: bool
}

/// In Money Map User
#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct InMoneyMapUserModel {
    pub email: Option<String>
}

/// MoneyMapUserModel Methods
impl MoneyMapUserModel{
    pub fn new(user: OutUserModel, owner: bool) -> MoneyMapUserModel{
        MoneyMapUserModel{
            user: Some(user),
            owner: owner
        }
    }

    /// Get User
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<ObjectId>' - id
    pub fn get_user(&self) -> Option<OutUserModel>{
        self.user.clone()
    }

    /// Get member owner flag
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'bool' - Owner flag
    pub fn is_owner(&self) -> bool{
        self.owner
    }
}

/// InMoneyMapUserModel Methods
impl InMoneyMapUserModel{
    pub fn new(email: String) -> InMoneyMapUserModel{
        InMoneyMapUserModel{
            email: Some(email)
        }
    }

    /// Get Email
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - The email
    pub fn get_email(&self) -> Option<String>{
        self.email.clone()
    }

    /// Validate Money Map
    ///
    /// # Arguments
    /// self
    /// user_option &Option<UserModel> The User to add to money map
    /// money_map &MoneyMapModel The money map
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self) -> ValidationResult{

        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.email.clone()){
            validation_result.add_error("email".to_string(), "Email is required.".to_string());
        }

        validation_result
    }//end validate
}
