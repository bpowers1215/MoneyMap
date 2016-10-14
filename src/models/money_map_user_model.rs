// src/models/money_map_user_model.rs

/// Money Map User Model

// Import Modules
// External
use ::bson::oid::ObjectId;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};
// Models
use ::models::money_map_model::{MoneyMapModel};
use ::models::user_model::{UserModel, OutUserModel};
// DAO
use ::dao::dao_manager::DAOManager;

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
}

/// InMoneyMapUserModel Methods
impl InMoneyMapUserModel{
    pub fn new(email: String) -> InMoneyMapUserModel{
        InMoneyMapUserModel{
            email: Some(email)
        }
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
    pub fn validate(&self, user_option: &Option<UserModel>, money_map: &MoneyMapModel) -> ValidationResult{

        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.email.clone()){
            validation_result.add_error("email".to_string(), "Email is required.".to_string());
        }
        // Verify email is unique
        if let &Some(ref user) = user_option {
            // A user has been found with this email address, verify the user isn't already a member of this money map
            let user_id = user.get_id().unwrap();
            if let Some(mm_users) = money_map.get_users(){
                for mm_user in mm_users{
                    if mm_user.get_user().unwrap().get_id().unwrap() == user_id {
                        validation_result.add_error("email".to_string(), "User already a member of this money map".to_string());
                        break;
                    }
                }
            }
        }else{
            validation_result.add_error("email".to_string(), "A user cannot be found with this email address.".to_string());
        }

        validation_result
    }//end validate
}
