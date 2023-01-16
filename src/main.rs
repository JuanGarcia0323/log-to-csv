use csv::WriterBuilder;
use polars::prelude::{CsvReader, CsvWriter, SerReader, SerWriter};
// use regex::Regex;
// use csv::Writer;
// use polars::prelude::*;
use regex::Regex;
use std::{
    fs::{self, File},
    // io::{BufWriter, Cursor, Write},
    io::Write,
};

fn main() {
    for file in fs::read_dir("C:/Users/Juan Emilio/Desktop/archivo_javi/files").unwrap() {
        let f = file.unwrap();
        let name = format!("{}.txt", f.file_name().to_str().unwrap());
        let path = f.path();
        // let name = format!("{}.txt", &path.to_str().unwrap());
        let re = Regex::new(r"API").unwrap();
        let mut file = File::create(&name).unwrap();
        let content = fs::read_to_string(path).unwrap();
        let lines = content.lines();
        let mut first = false;
        // let mut data: Vec<u8> = vec![];
        for l in lines {
            if re.is_match(l) {
                let result = if !first {
                    String::from(l)
                } else {
                    format!("\n{}", l)
                };
                first = true;
                file.write(result.as_bytes()).unwrap();
            }
        }
        // let result = Cursor::new(&data);
        // let mut df = CsvReader::new(result).finish().unwrap();
        // CsvWriter::new(data).finish(&mut df).unwrap();
        // break;
    }
}
