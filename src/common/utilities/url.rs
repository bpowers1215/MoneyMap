// src/common/utilities/url.rs

/// Utilities for URLs

// Import
// External
use ::regex::Regex;

#[derive(Debug)]
pub struct SortParam{
    pub field: String,
    pub direction: i32
}

/// Get Sort Param from URL string
///
/// # Arguments
/// * `sort` - A String slice of the "sort" property of a URL query String
/// sort string is expected to match the following regex: ([\+-]?)([^\|]+)
/// Examples: "date", "-date", "date|-name"
///
/// # Returns
/// A Vector of `SortParam`s
///
/// TODO: Add documentation test
pub fn get_sort_params(sort: &str) -> Vec<SortParam>{
    let mut sort_params = Vec::new();
    let re = Regex::new(r"(?P<direction>[\+-]?)(?P<field>[^\|]+)").unwrap();
    for caps in re.captures_iter(sort) {
        let mut direction = 1;
        let mut field = String::new();
        if let Some(dir) = caps.name("direction"){
            if dir == "-"{
                direction = -1;
            }
        }
        if let Some(fld) = caps.name("field"){
            field = fld.to_owned();
        }
        sort_params.push(SortParam{
            field: field,
            direction: direction
        });
    }

    sort_params
}
