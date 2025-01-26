use extendr_api::prelude::*;
use chrono::NaiveDate;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}


/// @export
#[extendr]
fn r_format_cdate(date_vec: Vec<String>, date_format: String) -> Vec<String> {
    date_vec.iter().map(|date_str| {
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => date.format(&date_format).to_string(),
            Err(_) => "Invalid date format".to_string(),
        }
    }).collect()
}

/// @export
#[extendr]
fn r_format_date(date_vec: Vec<i32>, date_format: String) -> Vec<String> {
    let r_epoch_offset = 719163; // Number of days from "0000-12-31" to "1970-01-01"
    
    date_vec.iter().map(|&days_since_r_epoch| {
        let adjusted_days = days_since_r_epoch + r_epoch_offset;
        match NaiveDate::from_num_days_from_ce_opt(adjusted_days) {
            Some(date) => date.format(&date_format).to_string(),
            None => "Invalid date format".to_string(),
        }
    }).collect()
}

/// @export
#[extendr]
pub fn standardise_strings(
    input: Vec<String>, 
    to_uppercase: bool, 
    trim_whitespace: bool, 
    remove_double_spaces: bool
) -> Vec<String> {
    input.into_iter().map(|s| {
        let mut result = s.to_string();

        // Convert to uppercase if required
        if to_uppercase {
            result = result.to_uppercase();
        }

        // Trim leading and trailing whitespace if required
        if trim_whitespace {
            result = result.trim().to_string();
        }

        // Remove occurrences of double spaces if required
        if remove_double_spaces {
            result = result.split_whitespace().collect::<Vec<&str>>().join(" ");
        }

        result
    }).collect()
}


extendr_module! {
    mod rust_fun;
    fn hello_world;
    fn standardise_strings;
    fn r_format_cdate;
    fn r_format_date;
}