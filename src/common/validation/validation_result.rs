// src/common/validation/validation_result.rs

/// Validation Result

use std::fmt;

///ValidationResult
pub struct ValidationResult{
    valid:bool,
    validations:Vec<FieldValidation>
}

impl FieldValidation{
    pub fn new() -> ValidationResult{
        ValidationResult{
            valid: true,
            validations:Vec::new()
        }
    }
    pub fn get_valid(&self) -> bool{
        self.valid
    }
    pub fn set_valid(&mut self, valid: bool){
        self.valid = valid;
    }
    pub fn get_validations(&self) -> Vec<FieldValidation>{
        self.validations
    }
}

///FieldValidation
pub struct FieldValidation{
    field:String,
    message:String
}

impl FieldValidation{
    pub fn get_field(&self) -> String{
        self.field
    }
    pub fn set_field(&mut self, field: String){
        self.field = field;
    }
    pub fn get_message(&self) -> String{
        self.field
    }
    pub fn set_message(&mut self, message: String){
        self.message = message
    }
}
