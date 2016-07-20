// src/models/test_model.rs

/// Test Model

//Import Modules
use ::rustc_serialize::json;

/// TestModel
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestModel {
    pub field_1: Option<String>,
    pub field_2: Option<String>,
    pub field_3: Option<String>
}
