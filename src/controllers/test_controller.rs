// src/controllers/test_controller.rs

/// Test Controller

//Import
//External
use nickel::{JsonBody, Request};
//Utilities
use ::dao::dao_manager::DAOManager;
use ::common::api_result::{ApiResult};
use ::common::data_access::ServerData;
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

    /// Imitate Saving a Test object
    /// Performs validation and returns result. Does not perform an actual save to the DB
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `APIResult` - Result
    pub fn save(&self, req: &mut Request<ServerData>) -> ApiResult<TestModel>{
        let result;
        match req.json_as::<TestModel>(){
            Ok(test) => {
                let validation_result = test.validate();

                if validation_result.is_valid(){
                    result = ApiResult::Success{result:test}
                }else{
                    result = ApiResult::Invalid{validation:validation_result, request:test}
                }
            },
            Err(e) => {
                error!("{}",e);
                result = ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
            }
        }
        result
    }//end save

    /// Imitate a hard failure
    /// Returns failure ApiResult
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `APIResult` - Result
    pub fn failure(&self) -> ApiResult<TestModel>{
        error!("(Test) Fatal error occurred");
        ApiResult::Failure{msg:"Fatal error occurred"}
    }

}
