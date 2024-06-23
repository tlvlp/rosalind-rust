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
    let mut buffer = vec![0; DEFAULT_BUFFER_SIZE];
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

pub fn with_read_all(solver: fn(&[u8]) -> Vec<u8>) {
    let mut buffer = vec![];
    let mut writer = writer();
    
    let full_size = reader().read_to_end(&mut buffer)
        .expect("Cannot read input file to String.");

    let result = solver(&buffer[..full_size]);

    writer.write_all(&result)
        .expect("Could not write partial result");
    writer.flush()
        .expect("Error while flushing buffer.");
}

pub fn check_ascii(b: u8) {
    if !b.is_ascii() {
        panic!("Input error: The following byte is not a valid ascii character: {b}")
    }
}
