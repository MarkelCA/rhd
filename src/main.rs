use anyhow::Context;
use clap::Parser;
use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read};

/// Simple program to greet a person
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

    const CHUNK_SIZE: usize = 16;

    let mut b = BufReader::with_capacity(CHUNK_SIZE, f);

    let mut line = 0;
    loop {
        let buffer = &mut [0;CHUNK_SIZE];

        let n = b.read(buffer).expect("Failed reading the byte buffer");

        if n == 0 {
            break;
        }

        let mut hex_string = String::new();

        for b in buffer.chunks(2) {
            let s = format!(
                "{:x}{:x} ",
                b.get(0).or(Some(&b'0')).unwrap(),
                b.get(1).or(Some(&b'0')).unwrap()
                );
            hex_string.push_str(&s)
        }

        println!("{:0>8x} {}", line*CHUNK_SIZE, hex_string);

        line+=1;
    }

    
    

}
