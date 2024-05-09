use std::io::{BufReader,Read};

const CHUNK_SIZE: usize = 16;

pub fn dump(f: std::fs::File) {
    let mut b = BufReader::with_capacity(CHUNK_SIZE, &f);
    let mut line = 0;
    loop {
        let buffer = &mut [0;CHUNK_SIZE];
        let bytes_read = b.read(buffer).expect("Failed reading the byte buffer");

        if bytes_read == 0 {
            break;
        }

        let mut hex_line   = String::new();
        let mut ascii_line = String::new();

        let line_bytes = buffer[..bytes_read].to_vec();

        for chunk in line_bytes.chunks(2) {
            let first_byte  = chunk.get(0).unwrap();
            let second_byte = chunk.get(1) ;

            let mut byte_pair_str = String::new();

            byte_pair_str.push_str(&format!("{:0>2x}",first_byte));
            ascii_line.push(get_printable(*first_byte));

            // If the file is odd the second byte will be None
            if let Some(byte) = second_byte {
                let second_byte_str = format!("{:x}",byte);
                byte_pair_str.push_str(&format!("{:0>2} ",second_byte_str));
                ascii_line.push(get_printable(*byte));

            }
            hex_line.push_str(format!("{}",byte_pair_str).as_str());
        }

        println!("{:0>8x}: {:<40} | {}", line*CHUNK_SIZE, hex_line,ascii_line);

        line+=1;
    }
}

fn get_printable(byte: u8) -> char {
    match byte {
        32..=127 => char::from_u32(byte as u32).unwrap(),
        _ => '.'
    }
}
