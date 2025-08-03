//! Excel Reader Library
//! 
//! A library for reading Excel files and converting them to Polars DataFrames.
//! Supports multi-header processing and worksheet selection.

pub mod config;
pub mod reader;
pub mod processor;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-export main types and functions for convenience
pub use config::ExcelConfig;
pub use reader::{get_worksheet_range, list_worksheets};
pub use processor::{process_excel, process_excel_worksheet};
pub use utils::{collapse_multi_headers, extract_data, process_headers};

// Re-export commonly used types
pub use polars::prelude::DataFrame;
pub use calamine::Data;
