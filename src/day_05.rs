use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub struct LineSpec {
    begin: (isize, isize),
    end: (isize, isize),
}

impl LineSpec {
    pub fn is_diagonal(self) -> bool {
        !(self.begin.0 == self.end.0 || self.begin.1 == self.end.1)
    }
    pub fn interpolate_line(self) -> HashSet<(isize, isize)> {
        let mut result = HashSet::new();

        let y_change: isize = self.begin.1 - self.end.1;
        let x_change: isize = self.begin.0 - self.end.0;

        let mut y = self.begin.1;
        let mut x = self.begin.0;

        let x_increment;
        let y_increment;

        if x_change != 0 && y_change != 0 {
            x_increment = if y_change < 0 { 1 } else { -1 };
            y_increment = if x_change < 0 { 1 } else { -1 };
        } else {
            let increment = if x_change < 0 || y_change < 0 { -1 } else { 1 };

            x_increment = if y_change == 0 { increment } else { 0 };
            y_increment = if x_change == 0 { increment } else { 0 };
        }

        loop {
            y = y - y_increment;
            x = x - x_increment;

            result.insert((x, y));

            if (x, y) == self.end {
                break;
            }
        }

        result.insert(self.begin);
        result.insert(self.end);

        result
    }
}

pub fn find_line_intersections(lines: Vec<LineSpec>) -> usize {
    let mut grid = HashMap::new();

    for line in lines {
        let line_points = line.interpolate_line();

        for p in line_points {
            let e = grid.entry(p).or_insert(0);
            *e += 1;
        }
    }

    grid.values().filter(|x| *x > &1).count()
}

#[cfg(test)]
mod tests {
    use crate::{aoc_utils::get_external_input, day_05::find_line_intersections};

    use super::LineSpec;

    #[test]
    fn part_one_sample_input_works() {
        let lines = get_external_input("day_05_test_input.txt", parse_line);

        let lines = lines.into_iter().filter(|x| !x.is_diagonal()).collect();

        let result = find_line_intersections(lines);

        assert_eq!(5, result);
    }

    #[test]
    fn part_one_external_input_works() {
        let lines = get_external_input("day_05_input.txt", parse_line);

        let lines = lines.into_iter().filter(|x| !x.is_diagonal()).collect();

        let result = find_line_intersections(lines);

        assert_eq!(6564, result);
    }

    #[test]
    fn part_two_sample_input_works() {
        let lines = get_external_input("day_05_test_input.txt", parse_line);

        let result = find_line_intersections(lines);

        assert_eq!(12, result);
    }

    fn parse_line(line: String) -> LineSpec {
        let coordinate_pair_raw = line
            .split("->")
            .into_iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let begin = coordinate_pair_raw[0]
            .split(",")
            .map(|s| s.parse::<isize>().expect("Guess this wasn't a number..."))
            .collect::<Vec<isize>>();

        let end = coordinate_pair_raw[1]
            .split(",")
            .map(|s| s.parse::<isize>().expect("Guess this wasn't a number..."))
            .collect::<Vec<isize>>();

        LineSpec {
            begin: (begin[0], begin[1]),
            end: (end[0], end[1]),
        }
    }
}
