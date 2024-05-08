use anyhow::Context;
use clap::Parser;
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
        let mut ascii_string = String::new();

        for chunk in buffer.chunks(2) {
            let first_byte  = chunk.get(0).unwrap_or(&b'0');
            let second_byte = chunk.get(1).unwrap_or(&b'0');

            if *first_byte as u8 == 0 && *second_byte as u8 == 0 {
                continue;
            }

            let mut s = format!("{:x}{:x} ",first_byte,second_byte);
            s = format!("{:0>5}",s);
            hex_string.push_str(&s);


            ascii_string.push(get_printable(*first_byte));
            ascii_string.push(get_printable(*second_byte));

            // ascii_string.push();
        }

        println!("{:0>8x}: {:<40} | {}", line*CHUNK_SIZE, hex_string,ascii_string);

        line+=1;
    }

    fn get_printable(byte: u8) -> char {
        match byte {
            32..=127 => char::from_u32(byte as u32).unwrap(),
            _ => '.'
        }
    }




}
