// src/controllers/test_controller.rs

/// Test Controller

//Import Modules
use ::rustc_serialize::json;
use ::bson::Bson;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::dao::dao_manager::DAOManager;
use ::dao::user_dao::UserDAO;
use ::common::validation::validation_result::{ValidationResult, FieldError};
use ::common::api_result::ApiResult;

// Nickel
//use nickel::{JsonBody, Request, Response};
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

pub struct TestController{
    dao_manager: DAOManager
}

impl TestController{

    pub fn new(dao_manager: DAOManager) -> TestController{
        TestController{
            dao_manager: dao_manager
        }
    }

    /// Retrieve a Success Response
    ///
    /// # Arguments
    /// req - nickel::Request
    ///
    /// # Returns
    /// `APIResult` - Result
    pub fn success(&self, req: &mut Request) -> ApiResult<String>{
        ApiResult::Success{result:"TEST".to_string()}
    }//end create_user

}
