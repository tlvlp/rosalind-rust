use std::io::{BufRead, Read, Write};
use crate::{Reader, Writer};

pub fn solve(mut reader: Reader, mut writer: Writer) {
    let buffer_size = 8 * 1024; //8KB
    let mut buffer = vec![0; buffer_size];

    loop {
        let read_bytes = reader.read(&mut buffer).expect("Cannot read input buffer!");
        if read_bytes == 0 {
            break;
        }
        // business logic
        let partial_result: Vec<u8> = buffer[..read_bytes]
            .iter()
            .map(|&b| {
                if !b.is_ascii() {
                    panic!("Input error: The following byte is not a valid ascii character: {b}")
                }
                if b as char == 'T' {
                    'U' as u8
                } else {
                    b
                }
            })
            .collect();

        // Write output
        writer.write_all(&partial_result)
            .expect("Could not write partial result");
    };

    writer.flush()
        .expect("Error while flushing buffer.");
}