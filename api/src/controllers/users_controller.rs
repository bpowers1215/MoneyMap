// src/controllers/users_controller.rs

/// Users Controller

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
use ::models::user_model::{InUserModel, LoginUserModel, OutUserModel, UserModel};

#[derive(Clone)]
pub struct UsersController{
    dao_manager: DAOManager,
    config: Config
}

impl UsersController{

    pub fn new(dao_manager: DAOManager, config: Config) -> UsersController{
        UsersController{
            dao_manager: dao_manager,
            config: config
        }
    }

    /// Find All Users
    /// ToDo: Remove this endpoint for security
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// `ApiResult<Vec<UserModel>>` - ApiResult including a vector of users
    pub fn find_all(&self) -> ApiResult<Vec<OutUserModel>, ()>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                info!("Fetch all Users");
                let users = dao.find(None);

                ApiResult::Success{result:users}
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end find_all

    /// Create New User
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<OutUserModel>` - ApiResult including the created user
    pub fn create(&self, req: &mut Request<ServerData>) -> ApiResult<OutUserModel, OutUserModel>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {

                match req.json_as::<InUserModel>(){
                    Ok(in_user) => {
                        // Validate User
                        let validation_result = in_user.validate_new(self.dao_manager.get_user_dao().unwrap());
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
                                    ApiResult::Failure{msg:"Unable to create user"}
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
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end create

    /// Modify User
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<OutUserModel>` - ApiResult including the modified user
    pub fn modify(&self, req: &mut Request<ServerData>) -> ApiResult<OutUserModel, OutUserModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_user_dao(){
            Ok(dao) => {

                match req.json_as::<InUserModel>(){
                    Ok(in_user) => {
                        // Validate User
                        let validation_result = in_user.validate_existing();
                        if validation_result.is_valid(){
                            let user = UserModel::new(in_user);
                            // Save User
                            match dao.update(user_id, &user){
                                Ok(result) => {
                                    if result.acknowledged && result.matched_count > 0  && result.modified_count == 0 {
                                        //Update found match, but no changes were needed to existing data
                                        ApiResult::Success{
                                            result:OutUserModel{ 
                                                id: None,
                                                first_name: None,
                                                last_name: None,
                                                email: None
                                            }
                                        }
                                    } else if result.acknowledged && result.matched_count > 0 && result.modified_count > 0 {
                                        //Update successful
                                        ApiResult::Success{result:OutUserModel::new(user)}
                                    }else{
                                        ApiResult::Failure{msg:"Update failed."}
                                    }
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
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end modify

    /// Log In
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<OutUserModel>` - ApiResult including the logged in user if login successful
    pub fn login(&self, req: &mut Request<ServerData>) -> ApiResult<LoginUserModel, LoginUserModel>{
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                // parse input
                match req.json_as::<InUserModel>(){
                    Ok(in_user) => {

                        // validate (require email and password)
                        let validation_result = in_user.login_validate();
                        if validation_result.is_valid() {
                            let filter = doc! {
                                "email": in_user.email.unwrap()
                            };

                            if let Some(found_user) = dao.find_one(Some(filter), None){
                                // Found user for email
                                if found_user.verify_password(in_user.password.unwrap()) {
                                    // Passwords match

                                    //Create a signed JASON Web Token (JWT) for authentication
                                    let header: Header = Default::default();

                                    // Define claims
                                    let claims = create_auth_claims(&self.config, found_user.get_id().unwrap().to_hex());

                                    let token = Token::new(header, claims);

                                    // Sign the token
                                    if let Some(ref auth_secret) = self.config.auth.auth_secret{
                                        match token.signed(auth_secret.as_bytes(), Sha256::new()){
                                            Ok(jwt) => {
                                                return ApiResult::Success{result:LoginUserModel::new(found_user, jwt)};
                                            }
                                            Err(e) => {
                                                error!("Login Error. Unable to sign JWT Token: {:?}.", e);
                                            }
                                        }
                                    }else{
                                        error!("Login Error. Unable to sign JWT Toke. No auth_secret key.");
                                    }
                                    return ApiResult::Failure{msg:"Unable to sign token for authentication."};
                                }
                            }
                            ApiResult::Failure{msg:"Invalid email address or password."}
                        }else{
                            ApiResult::Invalid{validation:validation_result, request:LoginUserModel::from_in_user(in_user)}
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
    }// end login

    /// Get current user details
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<OutUserModel>` - ApiResult including the logged in user
    pub fn get_account(&self, req: &mut Request<ServerData>) -> ApiResult<OutUserModel, ()>{
        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };
        match self.dao_manager.get_user_dao(){
            Ok(dao) => {
                match ObjectId::with_string(user_id.as_str()){
                    Ok(id) => {
                        let filter = doc! {
                            "_id" => id
                        };

                        if let Some(user) = dao.find_one(Some(filter), None){
                            ApiResult::Success{result:OutUserModel::new(user)}
                        }else{
                            ApiResult::Failure{msg:"Unable to find user."}
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to find user. Invalid ID."}
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end get_account

}// end impl UsersController

/// Get JWT auth claims for token
///
/// # Arguments
/// id - String The users ID
///
/// # Returns
/// `Registered` - The claims for the JWT token
fn create_auth_claims(config: &Config, id: String) -> Registered{
    let mut iss = String::new();
    let mut exp_duration = 1;// default expiration duration to 1 minute
    if let Some(ref claim_iss) = config.auth.claim_iss{
        iss = claim_iss.clone();
    }
    if let Some(ref exp_dur) = config.auth.exp_duration{
        exp_duration = exp_dur.clone();
    }
    let iat: DateTime<Local> = Local::now();
    let exp: DateTime<Local> = Local::now() + Duration::minutes(exp_duration);
    let claims = Registered {
        iss: Some(iss),
        sub: Some(id),
        exp: Some(exp.timestamp() as u64),
        iat: Some(iat.timestamp() as u64),
        ..Default::default()
    };
    claims
}// end create_auth_claims
