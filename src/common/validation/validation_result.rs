// src/common/validation/validation_result.rs

/// Validation Result

use std::fmt;

///ValidationResult
pub struct ValidationResult{
    valid:bool,
    results:Vec<FieldValidation>
}

impl ValidationResult{
    pub fn new() -> ValidationResult{
        ValidationResult{
            valid: true,
            results:Vec::new()
        }
    }
    pub fn get_valid(&self) -> bool{
        self.valid
    }
    pub fn set_valid(&mut self, valid: bool){
        self.valid = valid;
    }
    pub fn get_results(&self) -> Vec<FieldValidation>{
        self.results.clone()
    }
}

///FieldValidation
#[derive(Clone)]
pub struct FieldValidation{
    field:String,
    message:String
}

impl FieldValidation{
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
