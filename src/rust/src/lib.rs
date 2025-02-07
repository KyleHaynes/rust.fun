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


use regex::Regex;
use std::sync::Arc;
use std::thread;

/// @export
#[extendr]
fn match_vector(pattern: &str, strings: Vec<String>) -> Vec<bool> {
    // Compile the regex pattern
    let re = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(_) => return vec![false; strings.len()], // Return false for all if regex pattern is invalid
    };

    // Use rayon for parallel processing
    strings.par_iter()
        .map(|s| re.is_match(s))
        .collect()
}

use walkdir::{WalkDir, DirEntry};
use std::fs::Metadata;
use std::io;

fn is_file(entry: &DirEntry) -> bool {
    match entry.metadata() {
        Ok(metadata) => metadata.is_file(), // Check if it's a regular file
        Err(_) => {
            println!("Failed to get metadata for file: {}", entry.path().display()); // Print the file name
            false // Return false if metadata fetch fails
        }
    }
}

#[extendr]
fn list_files(dir: &str) -> Vec<String> {
    WalkDir::new(dir)
        .follow_links(true) // Follow symbolic links
        .into_iter()
        .filter_map(|entry| entry.ok()) // Skip errors while iterating
        .filter(|entry| is_file(entry)) // Ensure we only get regular files
        .map(|entry| entry.path().display().to_string())
        .collect()
}


#[extendr]
fn obj_size(obj: Robj) -> usize {
    obj.len() // This works for simple types like vectors but not for complex objects
}

#[extendr]
fn obj_memory_size(obj: Robj) -> Robj {
    // Call R's object.size() function properly
    R!("object.size(obj)", obj).unwrap()
}
use extendr_api::prelude::*;
use geo::{point, Polygon, MultiPolygon, Contains}; // Import the Contains trait
use geojson::{FeatureCollection, GeoJson};
use rayon::prelude::*;
use std::fs;

/// Load a GeoJSON file and find which polygon each lat-long point falls into, using Rayon for parallelism.
#[extendr]
fn assign_points_to_polygons(geojson_path: &str, lat: Vec<f64>, lon: Vec<f64>) -> extendr_api::Result<Vec<i32>> {
    // Input validation
    if lat.len() != lon.len() {
        return Err(extendr_api::Error::Other(
            "Latitude and longitude vectors must be of the same length.".to_string(),
        ));
    }

    // Step 1: Read GeoJSON file
    let geojson_str = fs::read_to_string(geojson_path).map_err(|e| {
        extendr_api::Error::Other(format!("Failed to read file: {}", e))
    })?;
    let geojson: GeoJson = geojson_str.parse().map_err(|e| {
        extendr_api::Error::Other(format!("Invalid GeoJSON: {}", e))
    })?;

    // Step 2: Extract polygons from the GeoJSON
    let polygons: Vec<Polygon<f64>> = match geojson {
        GeoJson::FeatureCollection(FeatureCollection { features, .. }) => {
            features
                .iter()
                .filter_map(|feature| feature.geometry.as_ref())
                .filter_map(|geometry| geo::Geometry::try_from(geometry).ok())
                .filter_map(|geometry| match geometry {
                    geo::Geometry::Polygon(p) => Some(vec![p]),  // Wrap individual Polygon in a Vec
                    geo::Geometry::MultiPolygon(mp) => Some(mp.0),  // Extract all Polygons from MultiPolygon
                    _ => None,
                })
                .flatten()
                .collect()
        }
        _ => {
            return Err(extendr_api::Error::Other(
                "GeoJSON must be a FeatureCollection.".to_string(),
            ))
        }
    };

    // Step 3: Use Rayon for parallel processing of points
    let result: Vec<i32> = lat
        .into_par_iter()
        .zip(lon.into_par_iter())
        .map(|(latitude, longitude)| {
            let pt = point!(x: longitude, y: latitude); // GeoJSON uses (lon, lat)
            polygons
                .iter()
                .enumerate()
                .find(|(_, poly)| poly.contains(&pt)) // Now this works because `Contains` is in scope
                .map(|(index, _)| index as i32) // Return polygon index
                .unwrap_or(-1) // -1 if no match found
        })
        .collect();

    Ok(result)
}
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
    fn match_vector;
    fn list_files;
    fn obj_memory_size;
    fn obj_size;
    fn assign_points_to_polygons;
    // fn compute_ratcliff_obershelp_distance;
}