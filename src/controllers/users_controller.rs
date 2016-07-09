// src/controllers/users_controller.rs

/// Users Controller

//Import Modules
use ::rustc_serialize::json;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::dao::dao_manager::DAOManager;
use ::dao::user_dao::UserDAO;

//Models
use ::models::user_model::{UserModel};

// Nickel
//use nickel::{JsonBody, Request, Response};
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

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
    /// `String` - JSON String response
    pub fn create(&self, req: &mut Request) -> String{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Create New User");

                let user = req.json_as::<UserModel>().unwrap();

                //Validate User
                user.validate();

                //Save User
                dao.create(&user);

                let response = json::encode(&user).unwrap();
                format!(r#"{{"status":"success", "data":{{"user":{}}}}}"#, response)
            },
            Err(e) =>{
                format!(r#"{{"status":"error", "msg":"{}""#, e)
            }
        }
    }//end create_user

}
