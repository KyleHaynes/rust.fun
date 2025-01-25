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
    date_vec.iter().map(|&days_since_epoch| {
        match NaiveDate::from_num_days_from_ce_opt(days_since_epoch) {
            Some(date) => date.format(&date_format).to_string(),
            None => "Invalid date format".to_string(),
        }
    }).collect()
}


extendr_module! {
    mod rust_fun;
    fn hello_world;
    fn r_format_cdate;
    fn r_format_date;
}