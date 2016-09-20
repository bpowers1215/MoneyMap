// src/common/authentication.rs

/// Authenticate the user (JWT Token)

// Import
// External
use chrono::{DateTime, Local};
use crypto::sha2::Sha256;
use hyper::header::{self, Authorization, Bearer};
use jwt::{Header, Registered, Token};
use nickel::{MiddlewareResult, Nickel, Response, Request};
use nickel::status::StatusCode::{Forbidden};
// Common Utilities
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::data_access::ServerData;
use ::common::config::Config;

/// Authentication Middleware
///
/// # Arguments
/// request - &mut Request<ServerData>
/// response - Response<'mw, ServerData>
///
/// # Returns
/// `MiddlewareResult<'mw, ServerData>`
pub fn authenticator<'mw>(request: &mut Request<ServerData>, response: Response<'mw, ServerData> ) -> MiddlewareResult<'mw, ServerData> {
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

            let token = match get_jwt_from_header(request){
                Ok(jwt_token) => jwt_token,
                Err(e) => {
                    return response.error(Forbidden, "Access denied. Unable to parse token.")
                }
            };

            if let Some(ref auth_secret) = server_data.config.auth.auth_secret{
                let secret = auth_secret.as_bytes();

                match validate_token(token, &secret, server_data.config.clone()){
                    Ok(_) => {
                        return response.next_middleware();
                    },
                    Err(e) => {
                        return response.error(Forbidden, e.get_message());
                    }
                }
            }else{
                error!("Authentication failure. Unable to verify JWT Token. No auth_secret key.");
            }
            response.error(Forbidden, "Access denied. Invalid token format.")
        }
    }
}// end authenticator

/// Retrieve the sub claim value from a JWT Token
/// Verify the signature before returning the value
///
/// # Arguments
/// token - Token<Header, Registered>
/// secret - &[u8]
///
/// # Returns
/// `MMResult<()>` Ok if token is verified, Err otherwise
pub fn get_session_id(request: &Request<ServerData>) -> MMResult<String>{
    let server_data: &ServerData = request.server_data();

    let token = match get_jwt_from_header(request){
        Ok(jwt_token) => jwt_token,
        Err(e) => {
            return Err(MMError::new(e.get_message(), MMErrorKind::Other));
        }
    };

    if let Some(ref auth_secret) = server_data.config.auth.auth_secret{
        let secret = auth_secret.as_bytes();

        return get_token_sub(token, &secret);
    }
    error!("Authentication failure. Unable to verify JWT Token. No auth_secret key.");

    Err(MMError::new("Token claim cannot be retrieved.", MMErrorKind::Other))
}// end get_session_id

/// Validate a JWT Token
/// Verify the signature and validate claims
///
/// # Arguments
/// token - Token<Header, Registered>
/// secret - &[u8]
/// config - Config
///
/// # Returns
/// `MMResult<()>` Ok if token is verified, Err otherwise
fn validate_token(token: Token<Header, Registered>, secret: &[u8], config: Config) -> MMResult<()>{
    // Verify the token
    if token.verify(secret, Sha256::new()) {
        // Verify The claims
        let current_time: DateTime<Local> = Local::now();

        // Verify Issuer claim
        if let Some(ref claim_iss) = config.auth.claim_iss{
            match token.claims.iss{
                Some(iss) => {
                    if iss.ne(claim_iss){
                        //return response.error(Forbidden, "Access denied. Invalid token issuer.");
                        return Err(MMError::new("Access denied. Invalid token issuer.", MMErrorKind::Other));
                    }
                },
                None => {
                    //return response.error(Forbidden, "Access denied. Token invalid. Claim iss is required).");
                    return Err(MMError::new("Access denied. Token invalid. Claim iss is required).", MMErrorKind::Other));
                }
            }
        }

        // Verify Issued-At claim
        if let Some(ref iat_ack_s) = config.auth.iat_ack{
            match token.claims.iat{
                Some(iat) => {
                    match DateTime::parse_from_rfc3339(iat_ack_s.as_str()){
                        Ok(iat_ack) => {
                            if iat as i64 <= iat_ack.timestamp(){
                                // Token was issued before iat_ack
                                //return response.error(Forbidden, "Access denied. Expired token (iat).");
                                return Err(MMError::new("Access denied. Expired token (iat).", MMErrorKind::Other));
                            }
                        },
                        Err(e) => {
                            error!("Authentication: iat_inception format invalid. {}", e);
                        }
                    }
                },
                None => {
                    //return response.error(Forbidden, "Access denied. Token invalid. Claim iat is required).");
                    return Err(MMError::new("Access denied. Token invalid. Claim iat is required).", MMErrorKind::Other));
                }
            }
        }

        // Verify Expiration claim
        match token.claims.exp{
            Some(exp) => {
                if exp as i64 <= current_time.timestamp(){
                    //return response.error(Forbidden, "Access denied. Expired token (exp).");
                    return Err(MMError::new("Access denied. Expired token (exp).", MMErrorKind::Other));
                }
            },
            None => {
                //return response.error(Forbidden, "Access denied. Token invalid. Claim exp is required).");
                return Err(MMError::new("Access denied. Token invalid. Claim exp is required).", MMErrorKind::Other));
            }
        }

        // Token signature and claims verified
        return Ok(());
    } else {
        //return response.error(Forbidden, "Access denied. Invalid token.");
        return Err(MMError::new("Access denied. Invalid token.", MMErrorKind::Other));
    }
}// end validate_token

/// Retrieve the JWT from request headers
///
/// # Arguments
/// request - &Request<ServerData>
///
/// # Returns
/// `MMResult<Token::<Header, Registered>>` The JWT token
fn get_jwt_from_header(request: &Request<ServerData>) -> MMResult<Token<Header, Registered>>{
    // Get the full Authorization header from the incoming request headers
    let auth_header = match request.origin.headers.get::<Authorization<Bearer>>() {
        Some(header) => header,
        None => {
            return Err(MMError::new("Token cannot be retrieved from header.", MMErrorKind::Other));
        }
    };

    // Format the header to only take the value
    let jwt = header::HeaderFormatter(auth_header).to_string();

    // Pass "Bearer" and retrieve just the token string
    let jwt_slice = &jwt[7..];

    // parse the JWT token
    match Token::<Header, Registered>::parse(jwt_slice){
        Ok(token) => Ok(token),
        Err(e) => {
            warn!("{:?}", e);
            Err(MMError::new("Unable to parse JWT Token.", MMErrorKind::Other))
        }
    }
}// end get_jwt_from_header

/// Retrieve the sub claim value from a JWT Token
/// Verify the signature before returning the value
///
/// # Arguments
/// token - Token<Header, Registered>
/// secret - &[u8]
///
/// # Returns
/// `MMResult<()>` Ok if token is verified, Err otherwise
fn get_token_sub(token: Token<Header, Registered>, secret: &[u8]) -> MMResult<String>{
    // Verify the token
    if token.verify(secret, Sha256::new()) {

        //return the sub claim
        if let Some(sub) = token.claims.sub{
            return Ok(sub);
        }
    }
    Err(MMError::new("Token claim cannot be retrieved.", MMErrorKind::Other))
}// end get_token_sub
