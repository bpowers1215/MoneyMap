// src/models/user_model.rs

/// User Model

//Import Modules
use ::rustc_serialize::json;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::bson::oid::ObjectId;

// Nickel
//use nickel::{JsonBody, Request, Response};
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

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
    /// 'MMResult<UserModel>' - the saved user
    pub fn validate(&self) -> MMResult<()>{
        //validate user

        Ok(())
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
