pub enum Vector {
    Forward(isize),
    Down(isize),
    Up(isize),
}

pub fn compute_current_position(vectors: Vec<Vector>) -> isize {
    let mut horizontal_position = 0;
    let mut vertical_position = 0;

    for v in vectors {
        match v {
            Vector::Forward(mag) => horizontal_position += mag,
            Vector::Down(mag) => vertical_position += mag,
            Vector::Up(mag) => vertical_position -= mag,
        }
    }

    horizontal_position * vertical_position
}

pub fn compute_current_position_with_aim(vectors: Vec<Vector>) -> isize {
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    let mut aim = 0;

    for v in vectors {
        match v {
            Vector::Forward(mag) => {
                horizontal_position += mag;
                vertical_position += mag * aim;
            },
            Vector::Down(mag) => aim += mag,
            Vector::Up(mag) => aim -= mag,
        }
    }

    horizontal_position * vertical_position
}

#[cfg(test)]
mod test {
    use crate::{aoc_utils::get_external_input, day_02::compute_current_position_with_aim};

    use super::{Vector, compute_current_position};

    #[test]
    fn part_one_sample_input_works() {
        let vectors = vec![
            Vector::Forward(5),
            Vector::Down(5),
            Vector::Forward(8),
            Vector::Up(3),
            Vector::Down(8),
            Vector::Forward(2),
        ];

        let result = compute_current_position(vectors);

        assert_eq!(150, result);
    }

    #[test]
    fn part_one_external_input_works() {
        let input = get_external_input("day_02_input.txt", parse_line);

        let result = compute_current_position(input);

        assert_eq!(1840243, result);
    }

    #[test]
    fn part_two_sample_input_works() {
        let vectors = vec![
            Vector::Forward(5),
            Vector::Down(5),
            Vector::Forward(8),
            Vector::Up(3),
            Vector::Down(8),
            Vector::Forward(2),
        ];

        let result = compute_current_position_with_aim(vectors);

        assert_eq!(900, result);
    }

    #[test]
    fn part_two_external_input_works() {
        let input = get_external_input("day_02_input.txt", parse_line);

        let result = compute_current_position_with_aim(input);

        assert_eq!(1727785422, result);
    }    

    fn parse_line(line: String) -> Vector {
        let components = line.split(" ").collect::<Vec<&str>>();
        let direction = components[0].trim();
        let magnitude = components[1].parse::<isize>()
            .expect("I guess that wasn't a number?");

        match direction {
            "forward" => Vector::Forward(magnitude),
            "down" => Vector::Down(magnitude),
            "up" => Vector::Up(magnitude),
            _ => panic!("Looks like there was something really wrong with that input chief."),
        }
    }
}
