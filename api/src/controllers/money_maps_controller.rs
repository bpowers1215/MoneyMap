// src/controllers/money_maps_controller.rs

/// Money Maps Controller

// Import
// External
use ::nickel::{JsonBody, Request};
use ::bson::{Bson};
use ::bson::oid::ObjectId;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::session as Session;
// Controllers
use ::controllers::money_map_users_controller::{MoneyMapUsersController};
// Models
use ::models::user_model::{OutUserModel};
use ::models::money_map_model::{MoneyMapModel, PubMoneyMapModel};
use ::models::money_map_user_model::{MoneyMapUserModel};
// DAO
use ::dao::dao_manager::DAOManager;

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
    /// `ApiResult<Vec<PubMoneyMapModel>>` - ApiResult including a vector of money maps
    pub fn find(&self, req: &mut Request<ServerData>) -> ApiResult<Vec<PubMoneyMapModel>, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };
        debug!("FIND MONEY MAPS. USER ID: {}", user_id);
        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {

                match ObjectId::with_string(&user_id){
                    Ok(user_obj_id) => {
                        //Get list of money maps for this user
                        let mut money_maps = dao.find(Some(doc!{
                            "users.user_id" => user_obj_id,
                            "deleted" => {
                                "$ne" => true
                            }
                        }));
                        debug!("# OF MONEY MAPS: {:?}",money_maps.len());
                        let mut pub_money_maps = Vec::new();


                        // Get list of user details for each money map
                        for i in 0..money_maps.len(){
                            debug!("MONEY MAP ID: {:?}",money_maps[i].get_id());
                            match MoneyMapUsersController::get_users_for_mm(&self.dao_manager, &money_maps[i]){
                                Ok(users_list) => {
                                    // Add the new list of user details to the money map
                                    money_maps[i].set_users(Some(users_list));
                                },
                                Err(e) => {
                                    return ApiResult::Failure{msg:e.get_message()};
                                }
                            }
                            pub_money_maps.push(PubMoneyMapModel::new(money_maps[i].clone()))
                        }

                        // Return the list of money maps
                        ApiResult::Success{result:pub_money_maps}
                    },
                    Err(e) => {
                        warn!("{}", e);
                        ApiResult::Failure{msg:"Could not find money maps. Invalid user ID."}
                    }
                }
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
    /// `ApiResult<PubMoneyMapModel, PubMoneyMapModel>` - ApiResult including the create money map
    pub fn create(&self, req: &mut Request<ServerData>) -> ApiResult<PubMoneyMapModel, PubMoneyMapModel>{

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

                        match req.json_as::<PubMoneyMapModel>(){
                            Ok(mut pub_money_map) => {
                                let mut money_map = MoneyMapModel::new(&pub_money_map);
                                // Validate
                                let validation_result = money_map.validate();
                                if validation_result.is_valid(){
                                    // Save Money Map
                                    match dao.create(&money_map, &user_id){
                                        Ok(result) => {
                                            // Set money map ID
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

                                            ApiResult::Success{result:PubMoneyMapModel::new(money_map)}
                                        },
                                        Err(e) => {
                                            error!("{}",e);
                                            ApiResult::Failure{msg:"Unable to create money map"}
                                        }
                                    }
                                }else{
                                    ApiResult::Invalid{validation:validation_result, request:pub_money_map}
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
    pub fn modify(&self, req: &mut Request<ServerData>) -> ApiResult<PubMoneyMapModel, PubMoneyMapModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {

                match req.json_as::<PubMoneyMapModel>(){
                    Ok(pub_edit_money_map) => {
                        let edit_money_map = MoneyMapModel::new(&pub_edit_money_map);

                        match ObjectId::with_string(&user_id){
                            Ok(user_obj_id) => {
                                if let Some(mm_id) = edit_money_map.get_id(){

                                    // Get Money Map
                                    let filter = doc!{
                                        "_id" => mm_id,
                                        "users.user_id" => user_obj_id
                                    };

                                    if let Some(_) = dao.find_one(Some(filter), None){

                                        // Validate
                                        let validation_result = edit_money_map.validate();
                                        if validation_result.is_valid(){
                                            // Save
                                            match dao.update(&edit_money_map){
                                                Ok(mut updated_mm) => {
                                                    match MoneyMapUsersController::get_users_for_mm(&self.dao_manager, &updated_mm){
                                                        Ok(users_list) => {
                                                            // Add the new list of user details to the money map
                                                            updated_mm.set_users(Some(users_list));
                                                            ApiResult::Success{result:PubMoneyMapModel::new(updated_mm)}
                                                        },
                                                        Err(e) => {
                                                            warn!("{}",e);
                                                            ApiResult::Success{result:PubMoneyMapModel::new(updated_mm)}
                                                        }
                                                    }
                                                },
                                                Err(e) => {
                                                    ApiResult::Failure{msg:e.get_message()}
                                                }
                                            }
                                        }else{
                                            ApiResult::Invalid{validation:validation_result, request:pub_edit_money_map}
                                        }

                                    }else{
                                        ApiResult::Failure{msg:"Unable to find Money Map"}
                                    }
                                }else{
                                    ApiResult::Failure{msg:"Unable to find Money Map. Invalid ID."}
                                }
                            },
                            Err(e) => {
                                error!("{}",e);
                                ApiResult::Failure{msg:"Unable to find Money Map. Invalid user ID."}
                            }
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
    /// req - &nickel::Request
    /// mm_id - String
    ///
    /// # Returns
    /// `ApiResult<String>` - ApiResult
    pub fn delete(&self, req: &Request<ServerData>, mm_id: &str) -> ApiResult<String, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {
                match dao.delete(&user_id, mm_id){
                    Ok(result) => {
                        if result.acknowledged && result.modified_count > 0 {
                            ApiResult::Success{result:"Successfully deleted money map".to_string()}
                        }else if result.acknowledged && result.matched_count == 0{
                            ApiResult::Failure{msg:"Unable to delete money map"}
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
}
