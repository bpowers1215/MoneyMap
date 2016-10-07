// src/controllers/money_maps_controller.rs

/// Money Maps Controller

// Import
// External
use ::chrono::{DateTime, Duration, Local};
use ::nickel::{JsonBody, Request};
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use ::std::default::Default;
use ::crypto::sha2::Sha256;
use ::jwt::{Header, Registered, Token};
use ::rustc_serialize::hex::ToHex;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::session as Session;
// DAO
use ::dao::dao_manager::DAOManager;
// Models
use ::models::user_model::{OutUserModel};
use ::models::money_map_model::{MoneyMapModel, MoneyMapUserModel};

#[derive(Clone)]
pub struct MoneyMapsController{
    dao_manager: DAOManager,
    config: Config
}

impl MoneyMapsController{

    pub fn new(dao_manager: DAOManager, config: Config) -> MoneyMapsController{
        MoneyMapsController{
            dao_manager: dao_manager,
            config: config
        }
    }

    /// Find All Money Maps belonging to the requesting user
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<Vec<MoneyMapModel>>` - ApiResult including a vector of money maps
    pub fn find(&self, req: &mut Request<ServerData>) -> ApiResult<Vec<MoneyMapModel>>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {
                //Get list of money maps for this user
                let mut money_maps = dao.find(Some(doc!{
                    "users.user_id" => user_id,
                    "deleted" => {
                        "$ne" => true
                    }
                }));


                // Get list of user details for each money map
                for i in 0..money_maps.len(){
                    match self.get_users_for_mm(&money_maps[i]){
                        Ok(users_list) => {
                            // Add the new list of user details to the money map
                            money_maps[i].set_users(Some(users_list));
                        },
                        Err(e) => {
                            return ApiResult::Failure{msg:e.get_message()};
                        }
                    }
                }

                // Return the list of money maps
                ApiResult::Success{result:money_maps}
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end find_all

    /// Create Money Map
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<MoneyMapModel>` - ApiResult including the create money map
    pub fn create(&self, req: &mut Request<ServerData>) -> ApiResult<MoneyMapModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {
                match self.dao_manager.get_user_dao(){
                    Ok(user_dao) => {

                        match req.json_as::<MoneyMapModel>(){
                            Ok(mut money_map) => {
                                // Validate
                                let validation_result = money_map.validate();
                                if validation_result.is_valid(){
                                    // Save User
                                    match dao.create(&money_map, user_id.clone()){
                                        Ok(result) => {
                                            // Set user ID
                                            match result.inserted_id{
                                                Some(id_wrapper) => {
                                                    match id_wrapper{
                                                        Bson::ObjectId(id) => money_map.set_id(id),
                                                        _ => {}
                                                    }
                                                },
                                                None => {}
                                            }
                                            // Add user details
                                            if let Ok(id) = ObjectId::with_string(user_id.as_str()){
                                                if let Some(user) = user_dao.find_one(Some(doc!{"_id" => id}), None){
                                                    money_map.set_users(Some(
                                                        vec![
                                                            MoneyMapUserModel::new(OutUserModel::new(user), true)
                                                        ]
                                                    ));
                                                }
                                            }

                                            ApiResult::Success{result:money_map}
                                        },
                                        Err(e) => {
                                            error!("{}",e);
                                            ApiResult::Failure{msg:"Unable to create money map"}
                                        }
                                    }
                                }else{
                                    ApiResult::Invalid{validation:validation_result, request:money_map}
                                }
                            },
                            Err(e) => {
                                error!("{}",e);
                                ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}",e.get_message().to_string());
                        ApiResult::Failure{msg:"Unable to interact with database"}
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end create

    /// Modify Money Map
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<MoneyMapModel>` - ApiResult including the modified user
    pub fn modify(&self, req: &mut Request<ServerData>) -> ApiResult<MoneyMapModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {

                match req.json_as::<MoneyMapModel>(){
                    Ok(edit_money_map) => {
                        // Get Money Map
                        if let Some(mm_id) = edit_money_map.get_id(){
                            let filter = doc!{
                                "_id" => mm_id,
                                "users.user_id" => user_id
                            };

                            if let Some(money_map) = dao.find_one(Some(filter), None){

                                // Validate
                                let validation_result = edit_money_map.validate();
                                if validation_result.is_valid(){
                                    // Save
                                    match dao.update(&edit_money_map){
                                        Ok(mut updated_mm) => {
                                            match self.get_users_for_mm(&updated_mm){
                                                Ok(users_list) => {
                                                    // Add the new list of user details to the money map
                                                    updated_mm.set_users(Some(users_list));
                                                    ApiResult::Success{result:updated_mm}
                                                },
                                                Err(e) => {
                                                    ApiResult::Success{result:updated_mm}
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            ApiResult::Failure{msg:e.get_message()}
                                        }
                                    }
                                }else{
                                    ApiResult::Invalid{validation:validation_result, request:edit_money_map}
                                }

                            }else{
                                ApiResult::Failure{msg:"Unable to find Money Map"}
                            }
                        }else{
                            ApiResult::Failure{msg:"Invalid Money Map ID"}
                        }
                    },
                    Err(e) => {
                        error!("{}",e);
                        ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end modify

    /// Delete a Money Map
    ///
    /// # Arguments
    /// &self
    /// id - String
    ///
    /// # Returns
    /// `ApiResult<String>` - ApiResult
    pub fn delete(&self, id: &str) -> ApiResult<String>{
        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {
                match dao.delete(id){
                    Ok(result) => {
                        if result.acknowledged && result.modified_count > 0 {
                            ApiResult::Success{result:"Successfully deleted money map".to_string()}
                        }else{
                            ApiResult::Failure{msg:"Unable to delete money map"}
                        }
                    },
                    Err(e) => {
                        error!("{}",e);
                        ApiResult::Failure{msg:"Malformed ID"}
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end delete

    fn get_users_for_mm(&self, money_map: &MoneyMapModel) -> MMResult<Vec<MoneyMapUserModel>>{
        // Initialze a list of user details for this money map
        let mut users_list = Vec::new();
        if let Some(mm_users) = money_map.get_users(){

            // For each user associated with this money map
            for mm_user in mm_users{
                match self.dao_manager.get_user_dao(){
                    Ok(user_dao) => {

                        // Fetch the user's details
                        let user_id = Bson::ObjectId(mm_user.user.unwrap().id.unwrap());
                        let found_user = user_dao.find_one(Some(doc!{
                            "_id" => user_id
                        }), None);
                        if let Some(user) = found_user{
                            // Add the user details to the list
                            users_list.push(
                                MoneyMapUserModel::new(OutUserModel::new(user), mm_user.owner)
                            );
                        }
                    },
                    Err(e) => {
                        error!("{}",e.get_message().to_string());
                        return Err(MMError::new("Unable to interact with database", MMErrorKind::Controller));
                    }
                }
            }
        }
        Ok(users_list)
    }
}
