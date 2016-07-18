// src/common/validation/validation_result.rs

/// Validation Result

use std::fmt;

///ValidationResult
pub struct ValidationResult{
    valid:bool,
    errors:Vec<FieldError>
}

impl ValidationResult{
    /// Create new ValidationResult
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn new() -> ValidationResult{
        ValidationResult{
            valid: true,
            errors:Vec::new()
        }
    }

    /// Get valid flag
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'bool' - True if valid, false otherwise
    pub fn get_valid(&self) -> bool{
        self.valid
    }

    /// Set valid flag
    ///
    /// # Arguments
    /// &mut self
    /// valid - bool
    pub fn set_valid(&mut self, valid: bool){
        self.valid = valid;
    }

    /// Get Errors
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'Vec<FieldError>' - Vector of FieldErrors
    pub fn get_errors(&self) -> Vec<FieldError>{
        self.errors.clone()
    }

    /// Add Field Error
    ///
    /// # Arguments
    /// &mut self
    /// field - String The Field name
    /// message - String The error message
    pub fn add_error(&mut self, field: String, message: String){
        self.errors.push(FieldError::new(field, message));
    }
}

///FieldError
#[derive(Clone)]
pub struct FieldError{
    field:String,
    message:String
}

impl FieldError{

    /// Create new FieldError
    ///
    /// # Arguments
    /// field - String The field name
    /// message - String The error message
    ///
    /// # Returns
    /// 'FieldError'
    pub fn new(field: String, message: String) -> FieldError{
        FieldError{
            field: field,
            message: message
        }
    }

    /// Get field name
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'String' The field name
    pub fn get_field(&self) -> &String{
        &self.field
    }

    /// Set field name
    ///
    /// # Arguments
    /// &mut self
    /// field - String The field name
    pub fn set_field(&mut self, field: String){
        self.field = field;
    }

    /// Get error message
    ///
    /// # Arguments
    /// &self
    ///
    /// # Returns
    /// 'String' The error message
    pub fn get_message(&self) -> &String{
        &self.field
    }

    /// Set error message
    ///
    /// # Arguments
    /// &mut self
    /// message - String the error message
    /// 'String' The field name
    pub fn set_message(&mut self, message: String){
        self.message = message
    }
}
