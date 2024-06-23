use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

const INPUT_PATH: &str = "input";
const RESULT_PATH: &str = "result";
pub const DEFAULT_BUFFER_SIZE: usize =  8 * 1024; // 8KB

fn reader() -> BufReader<File> {
    return BufReader::new(File::open(INPUT_PATH)
        .expect(format!("Cannot open input file: {INPUT_PATH}!").as_str()));
}

fn writer() -> BufWriter<File> {
    return BufWriter::new(File::create(RESULT_PATH)
        .expect(format!("Cannot create output file: {RESULT_PATH}!").as_str()));
}
pub fn with_default_continuous_buffer(solver: fn(&[u8]) -> Vec<u8>) {
    with_continuous_buffer(solver, DEFAULT_BUFFER_SIZE)
}

pub fn with_continuous_buffer(solver: fn(&[u8]) -> Vec<u8>, buffer_size: usize) {
    let mut buffer = vec![0; buffer_size];
    let mut reader = reader();
    let mut writer = writer();

    loop {
        let read_bytes = reader.read(&mut buffer)
            .expect("Cannot read input buffer!");
        if read_bytes == 0 { break; }

        let partial_result = solver(&buffer[..read_bytes]);

        writer.write_all(&partial_result)
            .expect("Could not write partial result");
    };

    writer.flush()
        .expect("Error while flushing buffer.");
}

pub fn check_ascii(b: u8) {
    if !b.is_ascii() {
        panic!("Input error: The following byte is not a valid ascii character: {b}")
    }
}
