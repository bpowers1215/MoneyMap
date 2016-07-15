// src/common/validation/validation_result.rs

/// Validation Result

use std::fmt;

///ValidationResult
pub struct ValidationResult{
    valid:bool,
    errors:Vec<FieldError>
}

impl ValidationResult{
    pub fn new() -> ValidationResult{
        ValidationResult{
            valid: true,
            errors:Vec::new()
        }
    }
    pub fn get_valid(&self) -> bool{
        self.valid
    }
    pub fn set_valid(&mut self, valid: bool){
        self.valid = valid;
    }
    pub fn get_errors(&self) -> Vec<FieldError>{
        self.errors.clone()
    }
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
    pub fn new(field: String, message: String) -> FieldError{
        FieldError{
            field: field,
            message: message
        }
    }
    pub fn get_field(&self) -> &String{
        &self.field
    }
    pub fn set_field(&mut self, field: String){
        self.field = field;
    }
    pub fn get_message(&self) -> &String{
        &self.field
    }
    pub fn set_message(&mut self, message: String){
        self.message = message
    }
}
