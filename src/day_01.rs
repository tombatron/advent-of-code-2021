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

pub fn sliding_window_depth_measurement_increase_counter(depth_readings: &Vec<usize>) -> usize {
    let mut sum_of_readings: Vec<usize> = Vec::new();

    let mut still_sliding = true;
    let mut offset = 0;

    while still_sliding {
        let sum = depth_readings.into_iter()
            .skip(offset)
            .take(3)
            .sum();

        sum_of_readings.push(sum);

        still_sliding = offset + 4 <= depth_readings.len();
        offset += 1;
    }

    depth_measurement_increase_counter(sum_of_readings)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use super::{
        depth_measurement_increase_counter, sliding_window_depth_measurement_increase_counter,
    };

    #[test]
    fn part_one_sample_input_works() {
        let sample_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let count = depth_measurement_increase_counter(sample_input);

        assert_eq!(7, count)
    }

    #[test]
    fn part_one_external_input_works() {
        let input = get_external_input("day_01_input.txt");

        let count = depth_measurement_increase_counter(input);

        assert_eq!(1215, count);
    }

    #[test]
    fn part_two_sample_input_works() {
        let sample_input = vec![607, 618, 618, 617, 647, 716, 769, 792];

        let count = sliding_window_depth_measurement_increase_counter(&sample_input);

        assert_eq!(5, count);
    }

    #[test]
    fn part_two_external_input_works() {
        let input = get_external_input("day_01_input.txt");

        let count = sliding_window_depth_measurement_increase_counter(&input);

        assert_eq!(1150, count);
    }

    fn get_external_input(file_name: &str) -> Vec<usize> {
        let day_one_input_file =
            File::open(file_name).expect("I guess that file didn't exist.");

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
}
