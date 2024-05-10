use anyhow::Context;
use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let f = File::open(&args.file_path)
        .context(format!("Reading file {}",args.file_path))
        .expect("Provided file not found");

    for line in rhd::dump(f) {
        println!("{}",line);
    }
}
