use clap::{App, Arg};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;

// --------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    name: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    email: Option<String>,
}

// --------------------------------------------------
fn run() -> Result<(), Box<dyn Error>> {
    let matches = App::new("csv-parser")
        .version("0.1.0")
        .about("CSV parser")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .comment(Some(b','))
        .comment(Some(b'.'))
        .from_path(file_path)?;

    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{:?}", err);
        std::process::exit(1);
    }
}
