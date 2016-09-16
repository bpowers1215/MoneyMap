// src/models/user_model.rs

/// User Model

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
use ::dao::user_dao::UserDAO;

/// User
#[derive(RustcDecodable, RustcEncodable)]
pub struct UserModel {
    pub id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct InUserModel {
    pub id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct OutUserModel {
    pub id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct LoginUserModel {
    pub id: Option<ObjectId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>
}

// User Model Methods
impl UserModel{

    /// Create a UserModel from InUserModel
    ///
    /// # Arguments
    /// in_user - InUserModel
    ///
    /// # Returns
    /// UserModel - The user
    pub fn new(in_user: InUserModel) -> UserModel{
        UserModel{
            id: in_user.id,
            first_name: in_user.first_name,
            last_name: in_user.last_name,
            email: in_user.email,
            password: if let Some(pwd_s) = in_user.password{
                if let Ok(pwd_h) = UserModel::hash_password(pwd_s){ Some(pwd_h)}else{ None }
            }else{
                None
            }
        }
    }// end new

    /// Hash password and base64 encode for storage
    ///
    /// # Arguments
    /// password - String
    ///
    /// # Returns
    /// 'MMResult<String>' - password base 64 encoded hashed password
    pub fn hash_password(password: String) -> MMResult<String>{
        //convert password to byte vector
        let pwd = password.as_bytes();

        //hash the password
        if let Ok(pwh) = pwhash::pwhash(pwd, pwhash::OPSLIMIT_INTERACTIVE, pwhash::MEMLIMIT_INTERACTIVE){
            //base64 encode byte vector
            let pwh_bytes = &pwh[..];
            let pwd_string = pwh_bytes.to_base64(Base64::STANDARD);
            Ok(pwd_string)
        }else{
            Err(MMError::new("Unable to hash password", MMErrorKind::Model))
        }
    }// end hash_password

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

    /// Hash password and base64 encode for storage
    ///
    /// # Arguments
    /// &self
    /// password: String The password to match
    ///
    /// # Returns
    /// bool - True if passwords match, false otherwise
    pub fn verify_password(&self, password: String) -> bool{

        if let Some(pws) = self.get_password(){

            // base64 decode password string
            if let Ok(pwb) = pws.from_base64(){
                // create HashedPassword for comparison
                if let Some(pwh) = HashedPassword::from_slice(&pwb){
                    // verify password hashes match
                    return pwhash::pwhash_verify(&pwh, password.as_bytes());
                }else{
                    error!("Unable to create HashedPassword from byte slice");
                }
            }
        }
        // no password stored for user
        false
    }// end hash_password

}

// In User Model Methods
impl InUserModel{

    /// Validate New User
    ///
    /// # Arguments
    /// self
    /// dao - UserDAO
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate_new(&self, dao: UserDAO) -> ValidationResult{

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
            if let Some(_) = dao.find_one(Some(filter), None){
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
                if password.as_str() != "" && confirm_password.as_str() != "" && !Validators::equals(password.clone(), confirm_password){
                    validation_result.add_error("confirm_password".to_string(), "Passwords must match.".to_string());
                }
            }
        }
        validation_result
    }//end validate_new

    /// Validate Existing User
    ///
    /// # Arguments
    /// self
    /// dao - UserDAO
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate_existing(&self, dao: UserDAO) -> ValidationResult{

        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::empty(&self.first_name){
            if !Validators::not_empty_string(self.first_name.clone()){
                validation_result.add_error("first_name".to_string(), "First Name is required.".to_string());
            }
        }
        if !Validators::empty(&self.last_name){
            if !Validators::not_empty_string(self.last_name.clone()){
                validation_result.add_error("last_name".to_string(), "Last Name is required.".to_string());
            }
        }
        if !Validators::empty(&self.email){
            validation_result.add_error("email".to_string(), "Email cannot be changed.".to_string());
        }
        if Validators::not_empty_string(self.password.clone()){
            // Verify confirm password and passwords match if a password is supplied
            if !Validators::not_empty_string(self.confirm_password.clone()){
                validation_result.add_error("confirm_password".to_string(), "Confirm Password is required.".to_string());
            }
            if let Some(password) = self.password.clone(){
                if let Some(confirm_password) = self.confirm_password.clone(){
                    if password.as_str() != "" && confirm_password.as_str() != "" && !Validators::equals(password.clone(), confirm_password){
                        validation_result.add_error("confirm_password".to_string(), "Passwords must match.".to_string());
                    }
                }
            }
        }
        validation_result
    }//end validate_existing

    /// Login Validate User
    /// Require email and password fields
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn login_validate(&self) -> ValidationResult{
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty_string(self.email.clone()){
            validation_result.add_error("email".to_string(), "Email is required.".to_string());
        }
        if !Validators::not_empty_string(self.password.clone()){
            validation_result.add_error("password".to_string(), "Password is required.".to_string());
        }
        validation_result
    }// end login_validate
}

// Pub User Model Methods
impl OutUserModel{

    /// Create a OutUserModel from UserModel
    ///
    /// # Arguments
    /// user - UserModel
    ///
    /// # Returns
    /// OutUserModel - The user
    pub fn new(user: UserModel) -> OutUserModel{
        OutUserModel{
            id:user.id,
            first_name:user.first_name,
            last_name:user.last_name,
            email:user.email
        }
    }// end new

    /// Create a OutUserModel from InUserModel
    ///
    /// # Arguments
    /// user - InUserModel
    ///
    /// # Returns
    /// OutUserModel - The user
    pub fn from_in_user(user: InUserModel) -> OutUserModel{
        OutUserModel{
            id:user.id,
            first_name:user.first_name,
            last_name:user.last_name,
            email:user.email
        }
    }// end from_in_user
}

// Login User Model Methods
impl LoginUserModel{

    /// Create a LoginUserModel from UserModel
    ///
    /// # Arguments
    /// user - UserModel
    ///
    /// # Returns
    /// LoginUserModel - The user
    pub fn new(user: UserModel, token: String) -> LoginUserModel{
        LoginUserModel{
            id:user.id,
            first_name:user.first_name,
            last_name:user.last_name,
            email:user.email,
            token: Some(token)
        }
    }// end new

    /// Create a LoginUserModel from InUserModel
    ///
    /// # Arguments
    /// user - InUserModel
    ///
    /// # Returns
    /// LoginUserModel - The user
    pub fn from_in_user(user: InUserModel) -> LoginUserModel{
        LoginUserModel{
            id:user.id,
            first_name:user.first_name,
            last_name:user.last_name,
            email:user.email,
            token:None
        }
    }// end from_in_user
}
