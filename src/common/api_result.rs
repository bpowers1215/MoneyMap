// src/common/api_result.rs

/// API Result
/// Represents an API result
/// Value types include Success, Error, Failure

//Import modules
use ::rustc_serialize::{Encodable, json};
use ::common::validation::validation_result::ValidationResult;

pub enum ApiResult<T, U>{
    Success{
        result: T
    },
    Invalid{
        validation: ValidationResult,
        request: U
    },
    Failure{
        msg: &'static str
    }
}

pub struct JsonEncoder;
impl JsonEncoder{
    pub fn encode<T: Encodable, U: Encodable>(api_result: &ApiResult<T, U>) -> String{
        match api_result{
            &ApiResult::Success{ref result} => {
                format!(r#"{{"status":"success", "data":{}}}"#, json::encode(&result).unwrap())
            },
            &ApiResult::Invalid{ref validation, ref request} => {
                format!(r#"{{"status":"invalid", "msg":"Request is invalid", "errors":{}, "request":{}}}"#, json::encode(&validation.get_errors()).unwrap(), json::encode(&request).unwrap())
            },
            &ApiResult::Failure{ref msg} => {
                format!(r#"{{"status":"error", "msg":"{}"}}"#, msg.to_string())
            }
        }
    }
}
