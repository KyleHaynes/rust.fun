use extendr_api::prelude::*;
use rayon::prelude::*;
use chrono::NaiveDate;

use text_distance::DamerauLevenshtein;
use text_distance::Levenshtein;
use text_distance::JaroWinkler;
use text_distance::Hamming;

use regex::Regex;
use std::sync::Arc;
use std::thread;

use walkdir::{WalkDir, DirEntry};
use std::fs::Metadata;
use std::io;

use geo::{point, Point, Polygon, MultiPolygon, Contains, BoundingRect, Geometry, Rect}; // Import the Contains trait
use geojson::{FeatureCollection, GeoJson};
use std::fs;
use serde_json::Value; // Import serde_json::Value for JSON value handling

use rand::Rng;
use std::collections::HashSet;


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

/// Load a GeoJSON file and find which polygon each lat-long point falls into, using Rayon for parallelism.
/// Returns the value of the specified property (e.g., "SA2_NAME21") for the matching polygon.
#[extendr]
fn assign_points_to_polygons(
    geojson_path: &str,
    lat: Vec<f64>,
    lon: Vec<f64>,
    property_name: Option<&str>, // Optional property name to return
) -> extendr_api::Result<Vec<String>> {
    // Input validation
    if lat.len() != lon.len() {
        return Err(extendr_api::Error::Other(
            "Latitude and longitude vectors must be of the same length.".to_string(),
        ));
    }

    // Default property name to return
    let property_name = property_name.unwrap_or("SA2_NAME21");

    // Step 1: Read GeoJSON file
    let geojson_str = fs::read_to_string(geojson_path).map_err(|e| {
        extendr_api::Error::Other(format!("Failed to read file: {}", e))
    })?;
    let geojson: GeoJson = geojson_str.parse().map_err(|e| {
        extendr_api::Error::Other(format!("Invalid GeoJSON: {}", e))
    })?;

    // Step 2: Extract polygons and properties from the GeoJSON
    let polygons_and_properties: Vec<(Polygon<f64>, Option<String>)> = match geojson {
        GeoJson::FeatureCollection(FeatureCollection { features, .. }) => {
            features
                .iter()
                .filter_map(|feature| {
                    // Extract geometry
                    let geometry = feature.geometry.as_ref()?;
                    let geo_geometry = geo::Geometry::try_from(geometry).ok()?;

                    // Extract property value
                    let property_value = feature
                        .property(property_name)
                        .and_then(|value| match value {
                            Value::String(s) => Some(s.clone()), // Use serde_json::Value::String
                            _ => None,
                        });

                    // Handle Polygon and MultiPolygon geometries
                    match geo_geometry {
                        geo::Geometry::Polygon(p) => Some(vec![(p, property_value)]),
                        geo::Geometry::MultiPolygon(mp) => Some(
                            mp.0.into_iter()
                                .map(|p| (p, property_value.clone()))
                                .collect(),
                        ),
                        _ => None,
                    }
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

    eprintln!("Extracted {} polygons from GeoJSON.", polygons_and_properties.len());

    // Step 3: Use Rayon for parallel processing of points
    let result: Vec<String> = lat
        .into_par_iter()
        .zip(lon.into_par_iter())
        .map(|(latitude, longitude)| {
            let pt = point!(x: longitude, y: latitude); // GeoJSON uses (lon, lat)
            // eprintln!("Processing point: ({}, {})", pt.x(), pt.y());

            // Find the first polygon that contains the point
            polygons_and_properties
                .iter()
                .find(|(polygon, _)| polygon.contains(&pt))
                .and_then(|(_, property_value)| property_value.as_ref().map(|v| v.clone()))
                .unwrap_or_else(|| "Unknown".to_string()) // Default value if no match or no property
        })
        .collect();

    Ok(result)
}


// Function to generate random lat-longs inside filtered polygons
#[extendr]
fn generate_random_lat_longs(geojson_path: &str, n: usize, property_name: &str, pattern: &str) -> Robj {
    let geojson_str = fs::read_to_string(geojson_path).expect("Failed to read file");
    let geojson: GeoJson = geojson_str.parse().expect("Invalid GeoJSON");
    let regex = Regex::new(pattern).expect("Invalid regex pattern");
    let mut polygons: Vec<Polygon<f64>> = Vec::new();

    // Extract polygons that match the property filter
    if let GeoJson::FeatureCollection(FeatureCollection { features, .. }) = geojson {
        for feature in features {
            if let Some(properties) = &feature.properties {
                if let Some(prop_value) = properties.get(property_name) {
                    if let Some(prop_str) = prop_value.as_str() {
                        if !regex.is_match(prop_str) {
                            continue; // Skip features that do not match
                        }
                    }
                }
            }

            // Convert geometry to polygons
            if let Some(geometry) = feature.geometry {
                if let Ok(geo) = Geometry::try_from(&geometry) {
                    match geo {
                        Geometry::Polygon(poly) => polygons.push(poly),
                        Geometry::MultiPolygon(mp) => polygons.extend(mp.0),
                        _ => continue, // Ignore other geometries
                    }
                }
            }
        }
    }

    if polygons.is_empty() {
        panic!("No matching polygons found.");
    }

    // Generate points inside polygons
    let mut rng = rand::thread_rng();
    let mut latitudes = Vec::new();
    let mut longitudes = Vec::new();
    let max_attempts = 1000; // Avoid infinite loops

    while latitudes.len() < n {
        for _ in 0..max_attempts {
            let polygon = &polygons[rng.gen_range(0..polygons.len())]; // Pick a random polygon
            let bbox = polygon.bounding_rect().unwrap();
            let x = rng.gen_range(bbox.min().x..bbox.max().x);
            let y = rng.gen_range(bbox.min().y..bbox.max().y);
            let point = Point::new(x, y);

            if polygon.contains(&point) {
                latitudes.push(y);
                longitudes.push(x);
                break; // Move to the next point
            }
        }

        if latitudes.len() >= n {
            break;
        }
    }

    // Return as an R list
    list!(
        lat = latitudes,
        lon = longitudes
    )
    .into()
}




/// Extract property names from a GeoJSON file.
#[extendr]
fn get_property_names(geojson_path: &str) -> extendr_api::Result<Vec<String>> {
    // Step 1: Read the GeoJSON file
    let geojson_str = fs::read_to_string(geojson_path).map_err(|e| {
        extendr_api::Error::Other(format!("Failed to read GeoJSON file: {}", e))
    })?;

    // Step 2: Parse the GeoJSON
    let geojson: GeoJson = geojson_str.parse().map_err(|e| {
        extendr_api::Error::Other(format!("Failed to parse GeoJSON: {}", e))
    })?;

    // Step 3: Extract property names
    let mut property_names = HashSet::new();
    match geojson {
        GeoJson::FeatureCollection(FeatureCollection { features, .. }) => {
            for feature in features {
                if let Some(properties) = &feature.properties {
                    for key in properties.keys() {
                        property_names.insert(key.clone());
                    }
                }
            }
        }
        _ => {
            return Err(extendr_api::Error::Other(
                "GeoJSON must be a FeatureCollection.".to_string(),
            ))
        }
    }

    // Step 4: Return the property names as a sorted vector
    let mut property_names: Vec<String> = property_names.into_iter().collect();
    property_names.sort();
    Ok(property_names)
}


extendr_module! {
    mod rust_fun;
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
    fn generate_random_lat_longs;
    fn get_property_names;
    // fn compute_ratcliff_obershelp_distance;
}