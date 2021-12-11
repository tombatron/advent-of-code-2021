use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_external_input(file_name: &str) -> Vec<usize> {
    let day_one_input_file = File::open(file_name).expect("I guess that file didn't exist.");

    let reader = BufReader::new(day_one_input_file);

    let mut result = Vec::new();

    for line in reader.lines().into_iter() {
        let line = line.expect("I guess I couldn't read that line...");
        let line_input = line
            .parse::<usize>()
            .expect("Line entry not parsable into usize.");

        result.push(line_input);
    }

    result
}
