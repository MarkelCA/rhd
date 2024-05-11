use std::io::{BufReader,Read, Seek};

const CHUNK_SIZE: usize = 16;

pub fn dump(f: std::fs::File) -> impl Iterator<Item = String>  {
    let mut b = BufReader::with_capacity(CHUNK_SIZE, f);

    std::iter::from_fn(move || {
        let buffer = &mut [0;CHUNK_SIZE];
        let line_offset = b.stream_position().expect("Failed getting the stream position");
        let bytes_read = b.read(buffer).expect("Failed reading the byte buffer");

        if bytes_read == 0 {
            return None;
        }

        let mut hex_line   = String::new();
        let mut ascii_line = String::new();

        let line_bytes = buffer[..bytes_read].to_vec();

        for chunk in line_bytes.chunks(2) {
            let first_byte  = chunk.get(0).unwrap(); // We should always have the first byte
            let second_byte = chunk.get(1);

            let mut byte_pair_str = String::with_capacity(2);

            byte_pair_str.push_str(&format!("{:0>2x}",first_byte));
            ascii_line.push(get_printable(*first_byte));

            // If the file is odd the second byte will be None at the last iteration
            if let Some(byte) = second_byte {
                byte_pair_str.push_str(&format!("{:0>2x} ",byte));
                ascii_line.push(get_printable(*byte));

            }
            hex_line.push_str(format!("{}",byte_pair_str).as_str());
        }

        let line_str = format!("{:0>8x}: {:<40}  {}", line_offset, hex_line,ascii_line);

        Some(line_str)
    })
}

fn get_printable(byte: u8) -> char {
    match byte {
        32..=127 => char::from(byte),
        _ => '.'
    }
}
