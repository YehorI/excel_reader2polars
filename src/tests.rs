//! Tests for the excel_reader library

#[cfg(test)]
mod tests {
    use crate::processor::process_excel_worksheet;
    use std::error;

    #[test]
    fn test_excel_processing() -> Result<(), Box<dyn error::Error>> {
        let path: &str = "src/test.xlsx";
        let worksheet_name: &str = "МАЙ  2024";
        let df = process_excel_worksheet(path, Some(worksheet_name), None)?;
        assert_eq!(df.shape().0, 2100);
        Ok(())
    }
}
