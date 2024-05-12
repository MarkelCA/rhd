use anyhow::Context;
use clap::Parser;
use std::io::{self, BufReader, Read};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file_path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let reader: BufReader<Box<dyn Read>> = match args.file_path {
        Some(file_path) => {
            let f = File::open(&file_path)
                .context(format!("Reading file {}", file_path))
                .expect("Provided file not found");
            BufReader::with_capacity(rhd::BATCH_CHUNK_SIZE, Box::new(f))
        },
        None => BufReader::with_capacity(rhd::BATCH_CHUNK_SIZE, Box::new(io::stdin())),
    };

    rhd::dump(reader);
}
