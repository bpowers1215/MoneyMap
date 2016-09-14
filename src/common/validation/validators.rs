// src/common/validation/validators.rs

/// Validators

/// Check that a field is not supplied, or None
///
/// # Arguments
/// val - Option<T> the Option field
///
/// # Returns
/// 'bool' - True if valid, false otherwise (None)
pub fn empty<T>(val: &Option<T>) -> bool{
    match val{
        &Some(_) => false,
        &None => true
    }
}

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

/// Check that two values are identical
///
/// # Arguments
/// val1 - Generic type T
/// val2 - Generic type T
///
/// # Returns
/// 'bool' - True if valid, false otherwise (None)
pub fn equals<T: PartialEq>(val1: T, val2: T) -> bool{
    val1 == val2
}
