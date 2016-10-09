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

/// Money Map User
#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct MoneyMapUserModel {
    pub user: Option<OutUserModel>,
    pub owner: bool
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
