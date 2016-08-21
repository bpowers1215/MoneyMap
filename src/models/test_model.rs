// src/models/test_model.rs

/// Test Model

//Import Modules
use ::common::validation::validators as Validators;
use ::common::validation::validation_result::{ValidationResult};

/// TestModel
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct TestModel {
    pub field_1: Option<String>,
    pub field_2: Option<String>,
    pub field_3: Option<String>
}


// Test Model Methods
impl TestModel{

    /// Validate TestModel
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// 'ValidationResult' - validation result
    pub fn validate(&self) -> ValidationResult{
        //validate user
        let mut validation_result = ValidationResult::new();
        if !Validators::not_empty(self.field_1.clone()){
            validation_result.add_error("field_1".to_string(), "field_1 is required.".to_string());
        }
        if !Validators::not_empty(self.field_2.clone()){
            validation_result.add_error("field_2".to_string(), "field_2 is required.".to_string());
        }
        if !Validators::not_empty(self.field_3.clone()){
            validation_result.add_error("field_3".to_string(), "field_3 is required.".to_string());
        }
        validation_result
    }//end validate

}
