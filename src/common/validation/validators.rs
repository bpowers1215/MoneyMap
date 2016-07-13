// src/common/validation/validators.rs

/// Validators

use std::fmt;

pub fn required<T>(val: Option<T>) -> bool{
    match val{
        Some(_) => true,
        None => false
    }
}
