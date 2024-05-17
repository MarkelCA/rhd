use anyhow::Context;
use clap::{value_parser,Parser};
use std::io::{self, BufReader, Read};
use std::fs::File;

use rhd::format::LineNumberFormat;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
   /// Input file to read from. If not provided reads from stdin.
   pub file_path: Option<String>,
   /// The format for the line numbers. Default is hexadecimal.
   #[clap(short, long, default_value = "hex", value_parser = value_parser!(LineNumberFormat))]
   pub format: LineNumberFormat,
}

impl CliArgs {
    fn to_args(self) -> rhd::Args  {
        rhd::Args {
            file_path: self.file_path,
            format: self.format,
        }
    }
}


fn main() {
    let args = CliArgs::parse();

    let reader: BufReader<Box<dyn Read>> = match args.file_path.clone() {
        Some(file_path) => {
            let f = File::open(&file_path)
                .context(format!("Reading file {}", file_path))
                .expect("Provided file not found");
            BufReader::with_capacity(rhd::BATCH_CHUNK_SIZE, Box::new(f))
        },
        None => BufReader::with_capacity(rhd::BATCH_CHUNK_SIZE, Box::new(io::stdin())),
    };

    let result = rhd::dump(reader, args.to_args());

    match result {
        Ok(_) => (),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::BrokenPipe => (),
                _ => eprintln!("Error: {}", e),
            }
        },
    }
}
