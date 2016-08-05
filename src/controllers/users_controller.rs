// src/controllers/users_controller.rs

/// Users Controller

//Import
//External
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use ::rustc_serialize::json;
use ::bson::Bson;
//Internal
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::validation::validation_result::{ValidationResult, FieldError};
use ::common::api_result::ApiResult;
use ::dao::dao_manager::DAOManager;
use ::dao::user_dao::UserDAO;
//Models
use ::models::user_model::{UserModel};
//Controllers
use ::controllers::controller_manager::ControllerManager;

#[derive(Clone)]
pub struct UsersController{
    dao_manager: DAOManager
}

impl UsersController{

    pub fn new(dao_manager: DAOManager) -> UsersController{
        UsersController{
            dao_manager: dao_manager
        }
    }

    /// Create New User
    ///
    /// # Arguments
    /// req - nickel::Request
    ///
    /// # Returns
    /// `MMResult<String>` - JSON String response
    pub fn create(&self, req: &mut Request<ControllerManager>) -> MMResult<String>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Create New User");

                let mut user = req.json_as::<UserModel>().unwrap();

                //Validate User
                let validation = user.validate();
                //if validation.get_valid(){
                    //Save User
                    match dao.create(&user){
                        Ok(result) => {
                            //Set user ID
                            match result.inserted_id{
                                Some(id_wrapper) => {
                                    match id_wrapper{
                                        Bson::ObjectId(id) => user.set_id(id),
                                        _ => {}
                                    }
                                },
                                None => {}
                            }

                            Ok(format!(r#"{{"user":{}}}"#, json::encode(&user).unwrap()))
                        },
                        Err(e) => Err(e)
                    }
                //}else{
                //    Err(format!(r#"{{"user":{}}}"#, json::encode(&user).unwrap()))
                //}
            },
            Err(e) => Err(e)
        }
    }//end create_user

}
