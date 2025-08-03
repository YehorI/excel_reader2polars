use clap::{Command, Arg};
use excel_reader::{process_excel_worksheet};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let matches = Command::new("Excel Reader")
        .version("1.0")
        .author("YehorI")
        .about("Excel worksheet processor")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .help("Path to the Excel file")
            .required(true))
        .arg(Arg::new("worksheet")
            .short('w')
            .long("worksheet")
            .help("Name of the worksheet to process")
            .required(false))
        .arg(Arg::new("header")
            .short('t')
            .long("header")
            .help("Header row number")
            .required(false))
        .get_matches();

    // Extract values from matches
    let path = matches.get_one::<String>("path").unwrap();
    let worksheet = matches
        .get_one::<String>("worksheet")
        .map(|s| s.as_str());
    let header_rows = matches
        .get_one::<String>("header")
        .map(|s| {
            s.split(',')
                .map(|v| v.trim().parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?;

    // Use the library function to process the Excel file
    let df = process_excel_worksheet(path, worksheet, header_rows)?;
    println!("{}", df.head(Some(10)));
    Ok(())
}
