pub fn compute_power_consumption(readings: &Vec<Vec<u8>>) -> usize {
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

fn binary_converter(accumulator: usize, value: &usize) -> usize {
    accumulator * 2 + value
}

#[cfg(test)]
mod test {
    use crate::{aoc_utils::get_external_input, day_03::compute_power_consumption};

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

    fn parse_line(line: String) -> Vec<u8> {
        let mut result = Vec::new();

        for c in line.chars() {
            let parsed_value = c.to_digit(10).expect("I guess that wasn't a digit?") as u8;

            result.push(parsed_value);
        }

        result
    }
}
