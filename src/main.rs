// src/main.rs

#[macro_use]
extern crate nickel;

extern crate rustc_serialize;
extern crate jwt;
extern crate hyper;
extern crate crypto;
extern crate chrono;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// Money Map Crate
extern crate money_map;

//Logging
#[macro_use]
extern crate log;
extern crate log4rs;

// Import
// External
use chrono::{DateTime, Local};
use crypto::sha2::Sha256;
use hyper::header::{self, Authorization, Bearer};
use jwt::{Header, Registered, Token};
use nickel::{MiddlewareResult, Nickel, Response, Request};
use nickel::status::StatusCode::{Forbidden};
// Common Utilities
use money_map::common::data_access::ServerData;
use money_map::common::database::DB;
use money_map::common::config::Config;
use money_map::common::routes;
//DAO
use money_map::dao::dao_manager::{DAOManager};
//Controllers
use money_map::controllers::controller_manager::ControllerManager;
use money_map::controllers::test_controller::TestController;
use money_map::controllers::users_controller::UsersController;

fn main() {
    //Setup logging
    log4rs::init_file("config/log.toml", Default::default()).unwrap();
    info!("Initializing API");

    //Load Configuration
    let configuration = Config::new();

    //Initialize Database Connection
    let db = match DB::new(configuration.clone()){
        Ok(db) => db,
        Err(e) => {
            //Cannot create database connection
            panic!("{}", e);
        }
    };

    //Initialize DAO Manager
    let dao_manager = DAOManager::new(db);

    //Initialize Controllers
    let controller_manager = ControllerManager{
        test_controller: TestController::new(dao_manager.clone()),
        users_controller: UsersController::new(dao_manager.clone(), configuration.clone())
    };

    //Initialize Data Access object
    let server_data = ServerData{
        config: configuration,
        controller_manager: controller_manager
    };

    let mut server = Nickel::with_data(server_data);

    server.utilize(authenticator);
    server.utilize(routes::get_routes());

    server.listen("0.0.0.0:6767");
}

fn authenticator<'mw>(request: &mut Request<ServerData>, response: Response<'mw, ServerData> ) -> MiddlewareResult<'mw, ServerData> {
    let server_data: &ServerData = request.server_data();

    // Check if we are getting an OPTIONS request
    if request.origin.method.to_string() == "OPTIONS".to_string() {
        // The middleware should not be used for OPTIONS, so continue
        response.next_middleware()
    } else {
        // We do not want to apply the middleware to the login route
        if request.origin.uri.to_string() == "/account/login".to_string() {
            response.next_middleware()
        } else {
            // Get the full Authorization header from the incoming request headers
            let auth_header = match request.origin.headers.get::<Authorization<Bearer>>() {
                Some(header) => header,
                None => {
                    return response.error(Forbidden, "Access denied. Authentication required.");
                }
            };

            // Format the header to only take the value
            let jwt = header::HeaderFormatter(auth_header).to_string();

            // We don't need the Bearer part,
            // so get whatever is after an index of 7
            let jwt_slice = &jwt[7..];

            // Parse the token
            if let Ok(token) = Token::<Header, Registered>::parse(jwt_slice){
                if let Some(ref auth_secret) = server_data.config.auth.auth_secret{
                    let secret = auth_secret.as_bytes();

                    // Verify the token
                    if token.verify(&secret, Sha256::new()) {
                        // Verify The claims
                        let current_time: DateTime<Local> = Local::now();

                        // Verify Issuer claim
                        if let Some(ref claim_iss) = server_data.config.auth.claim_iss{
                            match token.claims.iss{
                                Some(iss) => {
                                    if iss.ne(claim_iss){
                                        return response.error(Forbidden, "Access denied. Invalid token issuer.");
                                    }
                                },
                                None => {
                                    return response.error(Forbidden, "Access denied. Token invalid. Claim iss is required).");
                                }
                            }
                        }

                        // Verify Issued-At claim
                        if let Some(ref iat_ack_s) = server_data.config.auth.iat_ack{
                            match token.claims.iat{
                                Some(iat) => {
                                    match DateTime::parse_from_rfc3339(iat_ack_s.as_str()){
                                        Ok(iat_ack) => {
                                            if iat as i64 <= iat_ack.timestamp(){
                                                // Token was issued before iat_ack
                                                return response.error(Forbidden, "Access denied. Expired token (iat).");
                                            }
                                        },
                                        Err(e) => {
                                            error!("Authentication: iat_inception format invalid. {}", e);
                                        }
                                    }
                                },
                                None => {
                                    return response.error(Forbidden, "Access denied. Token invalid. Claim iat is required).");
                                }
                            }
                        }

                        // Verify Expiration claim
                        match token.claims.exp{
                            Some(exp) => {
                                if exp as i64 <= current_time.timestamp(){
                                    return response.error(Forbidden, "Access denied. Expired token (exp).");
                                }
                            },
                            None => {
                                return response.error(Forbidden, "Access denied. Token invalid. Claim exp is required).");
                            }
                        }

                        // Token signature and claims verified
                        return response.next_middleware();
                    } else {
                        return response.error(Forbidden, "Access denied. Invalid token.");
                    }
                }
                error!("Authentication failure. Unable to verify JWT Toke. No auth_secret key.");
            }
            response.error(Forbidden, "Access denied. Invalid token format.")
        }
    }
}
