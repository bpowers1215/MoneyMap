// src/common/validation/validators.rs

/// Validators

use std::fmt;

/// Check is a field has a value
///
/// # Arguments
/// val - Option<T> the Option field
///
/// # Returns
/// 'bool' - True if valid, false otherwise (None)
pub fn has_value<T>(val: Option<T>) -> bool{
    match val{
        Some(_) => true,
        None => false
    }
}
