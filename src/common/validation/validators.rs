// src/common/validation/validators.rs

/// Validators

use std::fmt;

/// Check that a field is not empty
///
/// # Arguments
/// val - Option<T> the Option field
///
/// # Returns
/// 'bool' - True if valid, false otherwise (None)
pub fn not_empty<T>(val: Option<T>) -> bool{
    match val{
        Some(_) => true,
        None => false
    }
}

/// Check that a String field is not empty
///
/// # Arguments
/// val - Option<String> the Option field
///
/// # Returns
/// 'bool' - True if valid, false otherwise (None)
pub fn not_empty_string(val: Option<String>) -> bool{
    match val{
        Some(v) => !v.is_empty(),
        None => false
    }
}
