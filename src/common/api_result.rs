// src/common/api_result.rs

/// API Result
/// Represents an API result
/// Value types include Success, Error, Failure

//Import modules
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
