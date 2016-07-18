// src/common/validation/validators.rs

/// Validators

use std::fmt;

/// Require and Option field
///
/// # Arguments
/// val - Option<T> the Option field
///
/// # Returns
/// 'bool' - True if valid, false otherwise (None)
pub fn required<T>(val: Option<T>) -> bool{
    match val{
        Some(_) => true,
        None => false
    }
}
