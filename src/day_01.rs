pub fn depth_measurement_increase_counter(depth_readings: Vec<usize>) -> usize {
    let mut counter = 0;
    let mut previous_reading = usize::MAX;

    for reading in depth_readings {
        if reading > previous_reading {
            counter = counter + 1;
        }

        previous_reading = reading;
    }

    counter
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use super::depth_measurement_increase_counter;

    #[test]
    fn sample_input_works() {
        let sample_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let count = depth_measurement_increase_counter(sample_input);

        assert_eq!(7, count)
    }

    #[test]
    fn external_input_works() {
        let input = get_external_input();

        let count = depth_measurement_increase_counter(input);

        assert_eq!(1215, count);
    }

    fn get_external_input() -> Vec<usize> {
        let day_one_input_file = File::open("day_01_input.txt")
            .expect("I guess that file didn't exist.");

        let reader = BufReader::new(day_one_input_file);

        let mut result = Vec::new();

        for line in reader.lines().into_iter() {
            let line = line.expect("I guess I couldn't read that line...");
            let line_input = line.parse::<usize>().expect("Line entry not parsable into usize.");

            result.push(line_input);
        }

        result
    }
}