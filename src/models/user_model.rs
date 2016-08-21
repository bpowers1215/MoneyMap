// src/models/user_model.rs

/// User Model

//Import Modules
use ::bson::oid::ObjectId;
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// User
#[derive(RustcDecodable, RustcEncodable)]
pub struct UserModel {
    id: Option<ObjectId>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    password: Option<String>
}

// User Model Methods
impl UserModel{

    /// Validate User
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self) -> ValidationResult{
        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.first_name.clone()){
            validation_result.add_error("first_name".to_string(), "First Name is required.".to_string());
        }
        if !Validators::not_empty_string(self.last_name.clone()){
            validation_result.add_error("last_name".to_string(), "Last Name is required.".to_string());
        }
        if !Validators::not_empty_string(self.email.clone()){
            validation_result.add_error("email".to_string(), "Email is required.".to_string());
        }
        validation_result
    }//end save

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

    /// Get First Name
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - first name
    pub fn get_first_name(&self) -> Option<String>{
        self.first_name.clone()
    }

    /// Get Last Name
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - last name
    pub fn get_last_name(&self) -> Option<String>{
        self.last_name.clone()
    }

    /// Get Email
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - email
    pub fn get_email(&self) -> Option<String>{
        self.email.clone()
    }

    /// Get Password
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Option<String>' - password
    pub fn get_password(&self) -> Option<String>{
        self.password.clone()
    }

}
