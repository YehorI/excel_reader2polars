# Excel Reader to Polars

A high-performance Rust library and CLI tool for reading Excel files and converting them to Polars DataFrames. Supports multi-header processing, worksheet selection, and parallel data processing.

## Features

- 📊 **Excel to DataFrame Conversion**: Convert Excel worksheets to Polars DataFrames
- 🏷️ **Multi-Header Support**: Handle complex Excel files with multiple header rows
- 📑 **Worksheet Selection**: Process specific worksheets by name
- ⚡ **High Performance**: Built with Rust and Rayon for parallel processing
- 🖥️ **CLI Interface**: Command-line tool for quick Excel processing
- 📚 **Library Interface**: Use as a library in your Rust projects

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

```bash
git clone <repository-url>
cd excel_reader2polars
cargo build --release
```

The compiled binary will be available at `target/release/excel_reader`.

## Usage

### Command Line Interface

Process an Excel file with the CLI:

```bash
# Basic usage - process first worksheet
./target/release/excel_reader -p path/to/file.xlsx

# Specify worksheet name
./target/release/excel_reader -p path/to/file.xlsx -w "Sheet1"

# Specify header rows (0-indexed, comma-separated)
./target/release/excel_reader -p path/to/file.xlsx -w "Sheet1" -t "0,1"
```

#### CLI Arguments

- `-p, --path <PATH>`: Path to the Excel file (required)
- `-w, --worksheet <NAME>`: Name of the worksheet to process (optional, defaults to first worksheet)
- `-t, --header <ROWS>`: Header row numbers, comma-separated (optional, 0-indexed)

### Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
excel_reader = { path = "path/to/excel_reader2polars" }
polars = "0.48.1"
```

#### Simple Usage

```rust
use excel_reader::process_excel_worksheet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Process first worksheet with default settings
    let df = process_excel_worksheet("data.xlsx", None, None)?;
    println!("{}", df.head(Some(5)));
    Ok(())
}
```

#### Advanced Configuration

```rust
use excel_reader::{ExcelConfig, process_excel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = ExcelConfig::new("data.xlsx")
        .with_worksheet("МАЙ 2024")
        .with_header_rows(vec![0, 1]);
    
    // Process with configuration
    let df = process_excel(&config)?;
    println!("{}", df);
    Ok(())
}
```

#### List Available Worksheets

```rust
use excel_reader::list_worksheets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worksheets = list_worksheets("data.xlsx")?;
    for worksheet in worksheets {
        println!("Worksheet: {}", worksheet);
    }
    Ok(())
}
```

## API Reference

### Core Functions

- `process_excel_worksheet(path, worksheet_name, header_rows)`: Simple function to process a worksheet
- `process_excel(config)`: Process Excel with advanced configuration
- `list_worksheets(path)`: Get list of available worksheets

### Configuration

- `ExcelConfig`: Configuration builder for Excel processing
  - `new(path)`: Create new config with file path
  - `with_worksheet(name)`: Set worksheet name
  - `with_header_rows(rows)`: Set header row indices

### Utilities

- `get_worksheet_range(path, worksheet)`: Get raw worksheet data range
- `collapse_multi_headers(headers)`: Combine multiple header rows
- `extract_data(range, start_row)`: Extract data from worksheet range
- `process_headers(range, header_rows)`: Process header information

## Dependencies

- **[calamine](https://crates.io/crates/calamine)** (0.26.0): Excel file reading
- **[polars](https://crates.io/crates/polars)** (0.48.1): Fast DataFrame library
- **[clap](https://crates.io/crates/clap)** (4.5.40): Command-line argument parsing
- **[rayon](https://crates.io/crates/rayon)** (1.10.0): Data parallelism
- **[thiserror](https://crates.io/crates/thiserror)** (2.0.12): Error handling
- **[tempfile](https://crates.io/crates/tempfile)** (3.20.0): Temporary file handling

## Examples

### Processing Multi-Header Excel Files

Many Excel files have complex header structures. This library can handle multiple header rows:

```rust
use excel_reader::{ExcelConfig, process_excel};

// Excel file with headers in rows 0 and 1
let config = ExcelConfig::new("complex_data.xlsx")
    .with_worksheet("Data")
    .with_header_rows(vec![0, 1]);

let df = process_excel(&config)?;
```

### Error Handling

```rust
use excel_reader::process_excel_worksheet;

match process_excel_worksheet("data.xlsx", Some("Sheet1"), None) {
    Ok(df) => println!("Success: {} rows, {} columns", df.height(), df.width()),
    Err(e) => eprintln!("Error processing Excel file: {}", e),
}
```

## Development

### Running Tests

```bash
cargo test
```

### Running with Debug Information

```bash
RUST_LOG=debug cargo run -- -p src/test.xlsx -w "Sheet1"
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Performance

This library is optimized for performance:

- **Parallel Processing**: Uses Rayon for concurrent data processing
- **Memory Efficient**: Streams data processing to minimize memory usage
- **Release Optimizations**: LTO and optimized codegen for production builds

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

YehorI

---

*Built with ❤️ in Rust*