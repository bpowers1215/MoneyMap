// src/models/user_model.rs

/// User Model

// Import Modules
// External
//use ::sodiumoxide::crypto::secretbox;
//use ::sodiumoxide::crypto::pwhash;
use ::bson::oid::ObjectId;
// Utilities
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};
// DAO
use ::dao::user_dao::UserDAO;

/// User
#[derive(RustcDecodable, RustcEncodable)]
pub struct UserModel {
    pub id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PubUserModel {
    pub id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

// User Model Methods
impl UserModel{

    /// Validate User
    ///
    /// # Arguments
    /// self
    /// dao - UserDAO
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self, dao: UserDAO) -> ValidationResult{

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
        // Verify email is unique
        if let Some(ref email) = self.email {
            let filter = doc!{
                "email" => email
            };
            if let Some(_) = dao.find(Some(filter), None){
                // A user has been found with this email address
                validation_result.add_error("email".to_string(), "This email is not available.".to_string());
            }
        }
        if !Validators::not_empty_string(self.password.clone()){
            validation_result.add_error("password".to_string(), "Password is required.".to_string());
        }
        if !Validators::not_empty_string(self.confirm_password.clone()){
            validation_result.add_error("confirm_password".to_string(), "Confirm Password is required.".to_string());
        }
        if let Some(password) = self.password.clone(){
            if let Some(confirm_password) = self.confirm_password.clone(){
                if password.as_str() != "" && confirm_password.as_str() != "" && !Validators::equals(password, confirm_password){
                    validation_result.add_error("confirm_password".to_string(), "Passwords must match.".to_string());
                }
            }
        }
        validation_result
    }//end validate

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

// Pub User Model Methods
impl PubUserModel{
    pub fn new(user: UserModel) -> PubUserModel{
        PubUserModel{
            id:user.id,
            first_name:user.first_name,
            last_name:user.last_name,
            email:user.email
        }
    }
}
