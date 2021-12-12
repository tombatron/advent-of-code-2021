pub fn compute_power_consumption(readings: &Vec<Vec<usize>>) -> usize {
    let mut i = readings[0].len() - 1;

    let mut gamma_bits = Vec::new();
    let mut epsilon_bits = Vec::new();

    // TODO: Refactor this to only require a single loop.
    loop {
        let mut ones = 0;
        let mut zeroes = 0;

        for row in readings {
            let reading = row[i];

            if reading == 0 {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }

        if ones > zeroes {
            gamma_bits.push(1);
            epsilon_bits.push(0);
        } else {
            gamma_bits.push(0);
            epsilon_bits.push(1);
        }

        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }

    let gamma = gamma_bits.iter().rev().fold(0, binary_converter);
    let epsilon = epsilon_bits.iter().rev().fold(0, binary_converter);

    gamma * epsilon
}

pub fn compute_life_support_rating(readings: &Vec<Vec<usize>>) -> usize {
    let oxygen_reading = filter_readings(readings.to_vec(), most_common, None);
    let co2_reading = filter_readings(readings.to_vec(), least_common, None);

    let oxygen = oxygen_reading
        .first()
        .expect("No readings?!")
        .to_vec()
        .iter()
        .fold(0, binary_converter);

    let co2 = co2_reading
        .first()
        .expect("No readings?!")
        .to_vec()
        .iter()
        .fold(0, binary_converter);

    oxygen * co2
}

fn filter_readings(
    readings: Vec<Vec<usize>>,
    filter: fn(MostCommon, usize, Vec<Vec<usize>>) -> Vec<Vec<usize>>,
    maybe_position: Option<usize>,
) -> Vec<Vec<usize>> {
    let position = maybe_position.unwrap_or(0);

    if readings.len() == 1 || readings[0].len() == position {
        readings
    } else {
        let most_common = find_most_common_bit(&readings, position);

        let filtered_readings = filter(most_common, position, readings);

        filter_readings(filtered_readings, filter, Some(position + 1))
    }
}

fn most_common(
    most_common: MostCommon,
    position: usize,
    readings: Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    match most_common {
        MostCommon::Equal | MostCommon::One => {
            readings.into_iter().filter(|r| r[position] == 1).collect()
        }
        MostCommon::Zero => readings.into_iter().filter(|r| r[position] == 0).collect(),
    }
}

fn least_common(
    most_common: MostCommon,
    position: usize,
    readings: Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    match most_common {
        MostCommon::Equal | MostCommon::One => {
            readings.into_iter().filter(|r| r[position] == 0).collect()
        }
        MostCommon::Zero => readings.into_iter().filter(|r| r[position] == 1).collect(),
    }
}

enum MostCommon {
    One,
    Zero,
    Equal,
}

fn find_most_common_bit(readings: &Vec<Vec<usize>>, position: usize) -> MostCommon {
    let mut zeroes = 0;
    let mut ones = 0;

    for reading in readings {
        match reading[position] {
            0 => zeroes += 1,
            1 => ones += 1,
            _ => (),
        }
    }

    if zeroes == ones {
        MostCommon::Equal
    } else if zeroes > ones {
        MostCommon::Zero
    } else {
        MostCommon::One
    }
}

fn binary_converter(accumulator: usize, value: &usize) -> usize {
    accumulator * 2 + value
}

#[cfg(test)]
mod test {
    use crate::{aoc_utils::get_external_input, day_03::compute_power_consumption};

    use super::compute_life_support_rating;

    #[test]
    fn part_one_sample_input_works() {
        let readings = vec![
            vec![0, 0, 1, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1],
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];

        let result = compute_power_consumption(&readings);

        assert_eq!(198, result);
    }

    #[test]
    fn part_one_external_input_works() {
        let input = get_external_input("day_03_input.txt", parse_line);

        let result = compute_power_consumption(&input);

        assert_eq!(1458194, result);
    }

    #[test]
    fn part_two_sample_input_works() {
        let readings = vec![
            vec![0, 0, 1, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1],
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];

        let result = compute_life_support_rating(&readings);

        assert_eq!(230, result);
    }

    #[test]
    fn part_two_external_input_works() {
        let input = get_external_input("day_03_input.txt", parse_line);

        let result = compute_life_support_rating(&input);

        assert_eq!(2829354, result);
    }    

    fn parse_line(line: String) -> Vec<usize> {
        let mut result = Vec::new();

        for c in line.chars() {
            let parsed_value = c.to_digit(10).expect("I guess that wasn't a digit?") as usize;

            result.push(parsed_value);
        }

        result
    }
}
