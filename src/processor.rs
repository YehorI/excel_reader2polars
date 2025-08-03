//! Main processing logic for Excel to DataFrame conversion

use crate::config::ExcelConfig;
use crate::reader::get_worksheet_range;
use crate::utils::{collapse_multi_headers, extract_data, process_headers};
use calamine::Data;
use polars::prelude::*;
use polars::error::PolarsError;
use rayon::prelude::*;
use std::error::Error;

/// Process an Excel worksheet and return a Polars DataFrame
/// 
/// # Arguments
/// 
/// * `config` - Configuration for processing the Excel file
/// 
/// # Returns
/// 
/// A Result containing the DataFrame or an error
/// 
/// # Example
/// 
/// ```
/// use excel_reader::{ExcelConfig, process_excel};
/// 
/// let config = ExcelConfig::new("src/test.xlsx")
///     .with_worksheet("МАЙ  2024")
///     .with_header_rows(vec![0]);
/// 
/// let df = process_excel(config)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn process_excel(config: ExcelConfig) -> Result<DataFrame, Box<dyn Error>> {
    process_excel_worksheet(
        &config.path,
        config.worksheet_name.as_deref(),
        config.header_rows,
    )
}

/// Process an Excel worksheet with the given parameters
/// 
/// # Arguments
/// 
/// * `path` - Path to the Excel file
/// * `worksheet_name` - Optional worksheet name (None for first worksheet)
/// * `header_rows` - Optional header row indices (defaults to [0])
/// 
/// # Returns
/// 
/// A Result containing the DataFrame or an error
pub fn process_excel_worksheet(
    path: &str,
    worksheet_name: Option<&str>,
    header_rows: Option<Vec<usize>>,
) -> Result<DataFrame, Box<dyn Error>> {
    let range = get_worksheet_range(path, worksheet_name)?;
    let header_rows = header_rows.unwrap_or(vec![0]);

    let rows: Vec<Vec<Data>> = range.rows().map(|row| row.to_vec()).collect();
    
    // Check header indices are in bounds
    for &idx in &header_rows {
        if idx >= rows.len() {
            return Err("One of header row indices is out of bounds".into());
        }
    }

    // Collect header rows
    let header_cells: Vec<&[Data]> = header_rows.iter().map(|&i| &rows[i][..]).collect();
    
    // Collapse headers
    let headers = collapse_multi_headers(&header_cells)?;

    // Data starts after the last header row
    let data_start = header_rows.iter().max().map(|x| x + 1).unwrap_or(1);
    let data_rows = &rows[data_start..];
    let data = extract_data(data_rows, headers.len());
    let df = create_dataframe(headers, data)?;
    Ok(df)
}

/// Create a Polars DataFrame from headers and data
/// 
/// # Arguments
/// 
/// * `headers` - Column headers
/// * `data` - Row data
/// 
/// # Returns
/// 
/// A Result containing the DataFrame or a PolarsError
fn create_dataframe(headers: Vec<String>, data: Vec<Vec<String>>) -> Result<DataFrame, PolarsError> {
    let headers = process_headers(headers);
    let columns: Vec<Column> = (0..headers.len())
        .into_par_iter()
        .map(|i| {
            let col_data: Vec<&str> = data.iter().map(|row| row[i].as_str()).collect();
            let series = Series::new(headers[i].clone().into(), &col_data);
            series.into()
        })
        .collect();
    Ok(DataFrame::new(columns)?)
}
