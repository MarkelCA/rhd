use std::io::{self,Read,Write};
use std::fmt::Write as FmtWrite;

pub const BATCH_CHUNK_SIZE: usize = 4096;
const LINE_CHUNK_SIZE: usize = 16;

pub mod format;

pub struct Args {
    pub file_path: Option<String>,
    pub format: format::LineNumberFormat,
}

pub fn dump<T: Read>(mut b: T, args: Args) {
    match args.format {
        format::LineNumberFormat::Hexadecimal => println!("hex!"),
        format::LineNumberFormat::Decimal => println!("dec!"),
    }
    let mut line = 0;
    let mut lock = io::stdout().lock();

    loop {
        let mut bytes_printed = 0;
        let buffer_chunk = &mut [0;BATCH_CHUNK_SIZE];
        let bytes_chunk = b.read(buffer_chunk).expect("Failed reading the byte buffer");

        if bytes_chunk == 0 {
            break;
        }

        let mut chunk_lines = String::new();

        for mut c in buffer_chunk.chunks(LINE_CHUNK_SIZE) {
            if bytes_printed as u64 >= bytes_chunk as u64 {
                break;
            }

            let line_length = 
                if bytes_printed + LINE_CHUNK_SIZE > bytes_chunk {bytes_chunk - bytes_printed} 
                else {LINE_CHUNK_SIZE};


            if bytes_printed + LINE_CHUNK_SIZE > bytes_chunk {
                c = &c[..line_length];
            }

            let mut hex_line   = String::new();
            let mut ascii_line = String::new();

            let line_bytes = c.to_vec();

            for chunk in line_bytes.chunks(2) {
                let first_byte  = chunk.get(0).unwrap(); // We should always have the first byte
                let second_byte = chunk.get(1);

                let mut byte_pair_str = String::with_capacity(2);

                write!(byte_pair_str,"{:0>2x}",first_byte).unwrap();
                ascii_line.push(get_printable(*first_byte));

                // If the file is odd the second byte will be None at the last iteration
                if let Some(byte) = second_byte {
                    write!(byte_pair_str,"{:0>2x} ",byte).unwrap();
                    ascii_line.push(get_printable(*byte));

                }
                hex_line.push_str(&byte_pair_str);
            }
            write!(chunk_lines,"{:0>8x}: {:<40}  {}\n", line*LINE_CHUNK_SIZE, hex_line,ascii_line).unwrap();
            line += 1;
            bytes_printed += line_length;
        }
        writeln!(lock,"{}",chunk_lines).unwrap();
    }
}


fn get_printable(byte: u8) -> char {
    match byte {
        32..=127 => char::from(byte),
        _ => '.'
    }
}
