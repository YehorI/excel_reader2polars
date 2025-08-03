//! Configuration types for Excel processing

/// Configuration for processing Excel worksheets
#[derive(Debug, Clone)]
pub struct ExcelConfig {
    /// Path to the Excel file
    pub path: String,
    /// Name of the worksheet to process (None for first worksheet)
    pub worksheet_name: Option<String>,
    /// Header row indices (0-based)
    pub header_rows: Option<Vec<usize>>,
}

impl ExcelConfig {
    /// Create a new ExcelConfig with the given path
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            worksheet_name: None,
            header_rows: None,
        }
    }

    /// Set the worksheet name
    pub fn with_worksheet(mut self, worksheet_name: impl Into<String>) -> Self {
        self.worksheet_name = Some(worksheet_name.into());
        self
    }

    /// Set the header row indices
    pub fn with_header_rows(mut self, header_rows: Vec<usize>) -> Self {
        self.header_rows = Some(header_rows);
        self
    }
}
