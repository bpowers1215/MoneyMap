// src/controllers/users_controller.rs

/// Users Controller

//Import
//External
use nickel::{JsonBody, Request};
use ::bson::Bson;
//Utilities
use ::common::api_result::ApiResult;
//DAO
use ::dao::dao_manager::DAOManager;
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
    
    /// Fetch All Users
    /// ToDo: Remove this endpoint for security
    ///
    /// # Arguments
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<Vec<UserModel>>` - ApiResult including a vector of users
    pub fn fetch_all(&self, req: &mut Request<ControllerManager>) -> ApiResult<Vec<UserModel>>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Fetch all Users");
                let users = dao.fetch_all();
                
                ApiResult::Success{result:users}
            },
            Err(e) => {
                error!("{}",e);
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }//end fetch_all

    /// Create New User
    ///
    /// # Arguments
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<UserModel>` - ApiResult including the created user
    pub fn create(&self, req: &mut Request<ControllerManager>) -> ApiResult<UserModel>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Create New User");

                match req.json_as::<UserModel>(){
                    Ok(mut user) => {
                        //Validate User
                        let validation_result = user.validate();
                        if validation_result.is_valid(){
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

                                    ApiResult::Success{result:user}
                                },
                                Err(e) => {
                                    error!("{}",e);
                                    ApiResult::Failure{msg:"Unable to save user"}
                                }
                            }
                        }else{
                            ApiResult::Invalid{validation:validation_result, request:user}
                        }
                    },
                    Err(e) => {
                        error!("{}",e);
                        ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
                    }
                }
            },
            Err(e) => {
                error!("{}",e);
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }//end create_user

}
