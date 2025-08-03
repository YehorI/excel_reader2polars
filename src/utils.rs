//! Utility functions for data processing

use calamine::Data;
use polars::prelude::*;
use std::error::Error;

/// Collapse multi-level headers into single header names
/// 
/// # Arguments
/// 
/// * `header_cells` - Vector of header row slices
/// 
/// # Returns
/// 
/// A Result containing the collapsed header names or an error
pub fn collapse_multi_headers(header_cells: &Vec<&[Data]>) -> Result<Vec<String>, Box<dyn Error>> {
    if header_cells.is_empty() {
        return Err("Empty header cells".into());
    }
    
    let cols = header_cells[0].len();
    let mut collapsed = Vec::with_capacity(cols);
    
    for col_idx in 0..cols {
        let parts: Vec<String> = header_cells
            .iter()
            .map(|row| row.get(col_idx).map(|d| d.to_string()).unwrap_or_default())
            .filter(|part| !part.starts_with("Unnamed") && !part.trim().is_empty())
            .collect();

        collapsed.push(if parts.is_empty() {
            format!("Unnamed_{}", col_idx)
        } else {
            parts.join(" ")
        });
    }
    Ok(collapsed)
}

/// Extract data from worksheet rows
/// 
/// # Arguments
/// 
/// * `data_rows` - Slice of data rows
/// * `header_len` - Expected number of columns
/// 
/// # Returns
/// 
/// A vector of rows, each containing string values
pub fn extract_data(data_rows: &[Vec<Data>], header_len: usize) -> Vec<Vec<String>> {
    data_rows
        .iter()
        .map(|row| {
            let mut cells: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
            cells.resize(header_len, String::new());
            cells
        })
        .collect()
}

/// Processes a vector of header names to ensure uniqueness.
///
/// This function takes a vector of strings representing header names and processes them
/// to ensure that each header name is unique. If a header name is empty, it is replaced
/// with a default name in the format "Unnamed_{index}", where {index} is the position
/// of the header in the input vector. If a header name already exists in the processed
/// list, a suffix is appended to the name to make it unique, following the format
/// "{base_name}_{suffix}".
///
/// # Parameters
///
/// - `headers`: A vector of strings containing the header names to be processed.
///
/// # Returns
///
/// A vector of strings containing the processed header names, ensuring that all names
/// are unique. If there were any duplicates or empty names in the input, they will be
/// modified accordingly.
///
/// # Example
///
/// ```
/// use excel_reader::process_headers;
/// 
/// let headers = vec!["Header1".to_string(), "".to_string(), "Header1".to_string()];
/// let processed = process_headers(headers);
/// assert_eq!(processed, vec!["Header1", "Unnamed_1", "Header1_1"]);
/// ```
///
/// # Panics
///
/// This function does not panic under normal circumstances, but it assumes that the
/// input vector is not excessively large, as it uses a hash set to track used names.
/// 
/// # Complexity
///
/// The function has a time complexity of O(n) where n is the number of headers, as it
/// iterates through the list and performs constant-time operations for each header.
pub fn process_headers(headers: Vec<String>) -> Vec<String> {
    let mut processed_headers = Vec::with_capacity(headers.len());
    let mut used_names = PlHashSet::new();

    for (i, header) in headers.iter().enumerate() {
        let base_name = if header.is_empty() {
            format!("Unnamed_{}", i)
        } else {
            header.clone()
        };

        let mut candidate = base_name.clone();
        let mut suffix = 0;

        // Generate a unique candidate name
        while used_names.contains(&candidate) {
            suffix += 1;
            candidate = format!("{}_{}", base_name, suffix);
        }

        used_names.insert(candidate.clone());
        processed_headers.push(candidate);
    }
    processed_headers
}
