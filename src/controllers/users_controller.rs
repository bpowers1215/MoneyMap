// src/controllers/users_controller.rs

/// Users Controller

// Import
// External
use nickel::{JsonBody, Request};
use ::bson::Bson;
// Utilities
use ::common::api_result::ApiResult;
// DAO
use ::dao::dao_manager::DAOManager;
// Models
use ::models::user_model::{InUserModel, OutUserModel, UserModel};
// Controllers
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
    pub fn find_all(&self, req: &mut Request<ControllerManager>) -> ApiResult<Vec<OutUserModel>>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Fetch all Users");
                let users = dao.find();
                
                ApiResult::Success{result:users}
            },
            Err(e) => {
                error!("{}",e);
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end fetch_all

    /// Create New User
    ///
    /// # Arguments
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<OutUserModel>` - ApiResult including the created user
    pub fn create(&self, req: &mut Request<ControllerManager>) -> ApiResult<OutUserModel>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Create New User");

                match req.json_as::<InUserModel>(){
                    Ok(mut in_user) => {
                        // Validate User
                        let validation_result = in_user.validate(self.dao_manager.get_user_dao().unwrap());
                        if validation_result.is_valid(){
                            let mut user = UserModel::new(in_user);
                            // Save User
                            match dao.create(&user){
                                Ok(result) => {
                                    // Set user ID
                                    match result.inserted_id{
                                        Some(id_wrapper) => {
                                            match id_wrapper{
                                                Bson::ObjectId(id) => user.set_id(id),
                                                _ => {}
                                            }
                                        },
                                        None => {}
                                    }

                                    ApiResult::Success{result:OutUserModel::new(user)}
                                },
                                Err(e) => {
                                    error!("{}",e);
                                    ApiResult::Failure{msg:"Unable to save user"}
                                }
                            }
                        }else{
                            ApiResult::Invalid{validation:validation_result, request:OutUserModel::from_in_user(in_user)}
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
    }// end create_user
    
    /// Log In
    ///
    /// # Arguments
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<OutUserModel>` - ApiResult including the logged in user if login successful
    pub fn login(&self, req: &mut Request<ControllerManager>) -> ApiResult<OutUserModel>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                // parse input
                match req.json_as::<InUserModel>(){
                    Ok(mut in_user) => {
                        
                        // validate (require email and password)
                        let validation_result = in_user.login_validate();
                        if validation_result.is_valid() {
                            info!("Login: Form valid");
                            let filter = doc!{
                                "email" => { in_user.email.unwrap() }
                            };
                            
                            if let Some(found_user) = dao.find_one(Some(filter), None){
                                info!("Login: found user for email");
                                if found_user.verify_password(in_user.password.unwrap()) {
                                    info!("Login: passwords match");
                                    return ApiResult::Success{result:OutUserModel::new(found_user)};
                                }
                            }
                            ApiResult::Failure{msg:"Invalid email address or password."}
                        }else{
                            ApiResult::Invalid{validation:validation_result, request:OutUserModel::from_in_user(in_user)}
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
    }// end login

}
