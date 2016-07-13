// src/controllers/users_controller.rs

/// Users Controller

//Import Modules
use ::rustc_serialize::json;
use ::bson::Bson;
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
    /// `MMResult<String>` - JSON String response
    pub fn create(&self, req: &mut Request) -> MMResult<String>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Create New User");

                let mut user = req.json_as::<UserModel>().unwrap();

                //Validate User
                user.validate();

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
            },
            Err(e) => Err(e)
        }
    }//end create_user

}
