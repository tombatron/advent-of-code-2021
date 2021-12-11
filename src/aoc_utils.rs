use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_external_input<T>(file_name: &str, result_parser: fn(String) -> T) -> Vec<T> {
    let input_file = File::open(file_name)
        .expect("I guess I couldn't open that file...");

    let reader = BufReader::new(input_file);

    let mut result = Vec::new();

    for line in reader.lines().into_iter() {
        let line = line.expect("I guess I couldn't read that line...");

        result.push(result_parser(line));
    }

    result
}
