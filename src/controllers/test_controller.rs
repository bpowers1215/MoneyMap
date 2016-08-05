// src/controllers/test_controller.rs

/// Test Controller

//Import
//External
use nickel::{JsonBody, Request, Response};
//Utilities
use ::dao::dao_manager::DAOManager;
use ::common::api_result::{ApiResult};
//Models
use ::models::test_model::{TestModel};
//Controllers
use ::controllers::controller_manager::ControllerManager;

#[derive(Clone)]
pub struct TestController{
    dao_manager: DAOManager
}

impl TestController{

    pub fn new(dao_manager: DAOManager) -> TestController{
        TestController{
            dao_manager: dao_manager
        }
    }

    /// Retrieve a Test
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `APIResult` - Result
    pub fn retrieve(&self) -> ApiResult<TestModel>{
        let test = TestModel{
            field_1: Some(String::from("Field One")),
            field_2: Some(String::from("Field Two")),
            field_3: Some(String::from("Field Three"))
        };
        ApiResult::Success{result:test}
    }//end retrieve

    /// "Save" a Test
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `APIResult` - Result
    pub fn save(&self, req: &mut Request<ControllerManager>) -> ApiResult<TestModel>{
        let test = TestModel{
            field_1: Some(String::from("Field One")),
            field_2: Some(String::from("Field Two")),
            field_3: Some(String::from("Field Three"))
        };
        ApiResult::Success{result:test}
    }//end save

}
