// src/models/money_map_user_model.rs

/// Money Map User Model

// Import Modules
// External
use ::bson::oid::ObjectId;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};
// Models
use ::models::user_model::{OutUserModel};
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
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self, dao_manager: &DAOManager) -> ValidationResult{

        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.email.clone()){
            validation_result.add_error("email".to_string(), "Email is required.".to_string());
        }
        // Verify email is unique
        match dao_manager.get_user_dao(){
            Ok(user_dao) => {
                match dao_manager.get_money_map_user_dao(){
                    Ok(mm_user_dao) => {
                        if let Some(ref email) = self.email {
                            let filter = doc!{
                                "email" => email
                            };
                            match user_dao.find_one(Some(filter), None){
                                Some(user) => {
                                    // A user has been found with this email address, verify the user isn't already a member of this money map
                                },
                                None => {
                                    validation_result.add_error("email".to_string(), "A user cannot be found with this email address.".to_string());
                                }
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}",e.get_message().to_string());
                        validation_result.add_error("email".to_string(), "Error: Unable to verify money map user.".to_string());
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                validation_result.add_error("email".to_string(), "Error: Unable to verify email address.".to_string());
            }
        }

        validation_result
    }//end validate
}
