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
    date_vec.par_iter().map(|date_str| {
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => date.format(&date_format).to_string(),
            Err(_) => "Invalid date format".to_string(),
        }
    }).collect()
}

use extendr_api::prelude::*;
use rayon::prelude::*;

/// @export

#[extendr]
fn r_format_date(date_vec: Vec<i32>, date_format: String) -> Vec<String> {
    let r_epoch_offset = 719163; // Number of days from "0000-12-31" to "1970-01-01"
    
    date_vec.par_iter().map(|&days_since_r_epoch| {
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
    input.par_iter().map(|s| {
        let mut result = s.clone();

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


use text_distance::DamerauLevenshtein;
use text_distance::Levenshtein;
use text_distance::JaroWinkler;
use text_distance::Hamming;

/// @export
#[extendr]
fn compute_damerau_levenshtein_distance(strs1: Vec<String>, strs2: Vec<String>) -> Vec<usize> {
    // Ensure both vectors are the same length
    if strs1.len() != strs2.len() {
        panic!("Input vectors must have the same length!");
    }

    strs1.iter()
        .zip(strs2.iter()) // Pair the elements from the two vectors
        .map(|(str1, str2)| {
            let damerau_levenshtein = DamerauLevenshtein {
                src: str1.to_string(),
                tar: str2.to_string(),
                restricted: false, // or true depending on your needs
            };
            damerau_levenshtein.distance() // Compute the distance for each pair
        })
        .collect() // Collect the results into a Vec<usize>
}

// Levenshtein distance
#[extendr]
fn compute_levenshtein_distance(strs1: Vec<String>, strs2: Vec<String>) -> Vec<usize> {
    if strs1.len() != strs2.len() {
        panic!("Input vectors must have the same length!");
    }

    strs1.iter()
        .zip(strs2.iter())
        .map(|(str1, str2)| {
            let lev = Levenshtein {
                src: str1.to_string(),
                tar: str2.to_string(),
            };
            lev.distance()
        })
        .collect()
}

// Jaro-Winkler distance
#[extendr]
fn compute_jaro_winkler_distance(strs1: Vec<String>, strs2: Vec<String>, winklerize: bool) -> Vec<f64> {
    if strs1.len() != strs2.len() {
        panic!("Input vectors must have the same length!");
    }

    strs1.par_iter()
        .zip(strs2.par_iter())  // Use parallel iterators
        .map(|(str1, str2)| {
            let jaro_winkler = JaroWinkler {
                src: str1.to_string(),
                tar: str2.to_string(),
                winklerize,
            };
            jaro_winkler.similarity()
        })
        .collect()
}


// Hamming distance
#[extendr]
fn compute_hamming_distance(strs1: Vec<String>, strs2: Vec<String>) -> Vec<usize> {
    if strs1.len() != strs2.len() {
        panic!("Input vectors must have the same length!");
    }

    strs1.iter()
        .zip(strs2.iter())
        .map(|(str1, str2)| {
            let hamming = Hamming {
                src: str1.to_string(),
                tar: str2.to_string(),
            };
            hamming.distance()
        })
        .collect()
}

// use text_distance::case::ratcliff_obershelp;

// #[extendr]
// fn compute_ratcliff_obershelp_distance(strs1: Vec<String>, strs2: Vec<String>) -> Vec<f64> {
//     if strs1.len() != strs2.len() {
//         panic!("Input vectors must have the same length!");
//     }

//     let ro = ratcliff_obershelp::default();  // Create an instance of RatcliffObershelp

//     strs1.par_iter()
//         .zip(strs2.par_iter())
//         .map(|(str1, str2)| {
//             ro.similarity(str1, str2) as f64  // Convert result to f64 for R compatibility
//         })
//         .collect()
// }

extendr_module! {
    mod rust_fun;
    fn hello_world;
    fn standardise_strings;
    fn r_format_cdate;
    fn r_format_date;
    fn compute_damerau_levenshtein_distance;
    fn compute_levenshtein_distance;
    fn compute_jaro_winkler_distance;
    fn compute_hamming_distance;
    // fn compute_ratcliff_obershelp_distance;
}