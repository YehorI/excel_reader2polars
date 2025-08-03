//! Excel file reading functionality

use calamine::{Data, Range, Reader, Xlsx, open_workbook};
use std::error::Error;

/// Get the range of a specific worksheet or the first worksheet
/// 
/// # Arguments
/// 
/// * `path` - Path to the Excel file
/// * `worksheet_name` - Optional worksheet name (None for first worksheet)
/// 
/// # Returns
/// 
/// A Result containing the Range or an error
pub fn get_worksheet_range(
    path: &str,
    worksheet_name: Option<&str>,
) -> Result<Range<Data>, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let range = match worksheet_name {
        Some(name) => workbook.worksheet_range(name)?,
        None => {
            // Get the first worksheet
            let sheets = workbook.worksheets();
            if sheets.is_empty() {
                return Err("No worksheets found in the workbook".into());
            }

            // Clone the range from the first worksheet
            // sheets[0] contains a tuple of (name, range)
            sheets[0].1.clone()
        }
    };

    Ok(range)
}

/// List all worksheet names in an Excel file
/// 
/// # Arguments
/// 
/// * `path` - Path to the Excel file
/// 
/// # Returns
/// 
/// A Result containing a vector of worksheet names or an error
pub fn list_worksheets(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let sheets = workbook.worksheets();
    Ok(sheets.into_iter().map(|(name, _)| name).collect())
}
