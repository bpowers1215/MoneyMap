// src/common/api_result.rs

/// API Result
/// Represents an API result
/// Value types include Success, Error, Failure

//Import modules
use ::rustc_serialize::{Encodable, json};
use ::common::validation::validation_result::ValidationResult;

pub enum ApiResult<T>{
    Success{
        result: T
    },
    Error{
        result: ValidationResult,
        request: T
    }
}

pub struct JsonEncoder;
impl JsonEncoder{
    pub fn encode<T: Encodable>(api_result: &ApiResult<T>) -> String{
        match api_result{
            &ApiResult::Success{ref result} => {
                format!(r#"{{"status":"success", "data":{}}}"#, json::encode(&result).unwrap())
            },
            &ApiResult::Error{ref result, ref request} => {
                format!(r#"{{"status":"error", "msg":""}}"#)
            }
        }
    }
}
