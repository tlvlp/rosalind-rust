mod _01_rna;

use std::fs::File;
use std::io::{BufReader, BufWriter};

const INPUT_PATH: &str = "input";
const RESULT_PATH: &str = "result";

type Reader = BufReader<File>;
type Writer = BufWriter<File>;

fn main() {
    let reader: Reader = BufReader::new(File::open(INPUT_PATH)
        .expect(format!("Cannot open input file: {INPUT_PATH}!").as_str()));
    let writer: Writer = BufWriter::new(File::create(RESULT_PATH)
        .expect(format!("Cannot create output file: {RESULT_PATH}!").as_str()));

    _01_rna::solve(reader, writer)
}
