// src/models/money_map_dao.rs

/// Money Map Model

// Import Modules
// External
use ::bson::oid::ObjectId;
use ::rustc_serialize::base64 as Base64;
use ::rustc_serialize::base64::{FromBase64, ToBase64};
use ::sodiumoxide::crypto::pwhash;
use ::sodiumoxide::crypto::pwhash::HashedPassword;
// Utilities
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};
// DAO
use ::dao::money_map_dao::MoneyMapDAO;

/// Money Map
#[derive(RustcDecodable, RustcEncodable)]
pub struct MoneyMapModel {
    pub id: Option<ObjectId>,
    pub name: Option<String>
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
