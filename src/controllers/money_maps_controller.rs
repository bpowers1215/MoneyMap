// src/controllers/money_maps_controller.rs

/// Money Maps Controller

// Import
// External
use ::chrono::{DateTime, Duration, Local};
use ::nickel::{JsonBody, Request};
use ::bson::Bson;
use ::bson::oid::ObjectId;
use ::std::default::Default;
use ::crypto::sha2::Sha256;
use ::jwt::{Header, Registered, Token};
use ::rustc_serialize::hex::ToHex;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::session as Session;
// DAO
use ::dao::dao_manager::DAOManager;
// Models
use ::models::money_map_model::{MoneyMapModel};

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
        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {
                let money_maps = dao.find();

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
        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {

                match req.json_as::<MoneyMapModel>(){
                    Ok(mut money_map) => {
                        // Validate
                        let validation_result = money_map.validate();
                        if validation_result.is_valid(){
                            // Save User
                            match dao.create(&money_map){
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
    }// end create

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
                        ApiResult::Failure{msg:"Delete failed"}
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
