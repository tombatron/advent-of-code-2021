use std::collections::HashMap;

enum BingoBoardResult {
    Winner(usize),
    NotWinner,
}

#[derive(Clone, Copy)]
struct BingoBoard {
    board: [usize; 25],
}

impl BingoBoard {
    pub fn new(items: [usize; 25]) -> Self {
        BingoBoard { board: items }
    }

    pub fn check_winner(self, marked_numbers: &Vec<usize>) -> BingoBoardResult {
        if marked_numbers.len() < 5 {
            return BingoBoardResult::NotWinner;
        }

        let mut x_counts = HashMap::new();
        let mut y_counts = HashMap::new();

        let mut marked_score = 0;

        for marked_number in marked_numbers {
            let index = self.board.iter().position(|r| r == marked_number);

            match index {
                Some(i) => {
                    marked_score += marked_number;

                    let x = i % 5;
                    let y = i / 5;

                    let xc = x_counts.entry(x).or_insert(0);
                    *xc += 1;

                    let yc = y_counts.entry(y).or_insert(0);
                    *yc += 1;
                }

                None => (),
            }
        }

        if x_counts.values().into_iter().any(|x| *x == 5)
            || y_counts.values().into_iter().any(|y| *y == 5)
        {
            let board_total: usize = self.board.iter().sum();
            let score = board_total - marked_score;

            let final_score = score * marked_numbers.last().unwrap();

            return BingoBoardResult::Winner(final_score);
        }

        BingoBoardResult::NotWinner
    }
}

struct BingoBoardCollection {
    pub boards: Vec<BingoBoard>,
}

impl BingoBoardCollection {
    pub fn new(boards: Vec<BingoBoard>) -> Self {
        BingoBoardCollection { boards: boards }
    }

    pub fn mark_numbers(&self, numbers: &Vec<usize>) -> Option<usize> {
        let bs = self.boards.to_vec();

        let mut i = 5;

        loop {
            if i == numbers.len() {
                break;
            }

            for b in &bs {
                match b.check_winner(&numbers[0..i].to_vec()) {
                    BingoBoardResult::Winner(score) => return Some(score),
                    BingoBoardResult::NotWinner => (),
                }
            }

            i += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use super::{BingoBoard, BingoBoardCollection};

    #[test]
    fn part_one_sample_input_works() {
        let input_file_name = "day_04_test_input.txt";

        let drawn_numbers = get_number_sequence(input_file_name);
        let boards = get_game_boards(input_file_name);

        let result = boards.mark_numbers(&drawn_numbers).unwrap();

        assert_eq!(4512, result);
    }

    #[test]
    fn part_one_external_input_works() {
        let input_file_name = "day_04_input.txt";

        let drawn_numbers = get_number_sequence(input_file_name);
        let boards = get_game_boards(input_file_name);

        let result = boards.mark_numbers(&drawn_numbers).unwrap();

        assert_eq!(4512, result);
    }

    fn get_number_sequence(file_name: &str) -> Vec<usize> {
        let input_file = File::open(file_name).expect("I guess I couldn't open that file...");

        let mut reader = BufReader::new(input_file);

        let mut first_line = String::new();

        let _ = reader.read_line(&mut first_line);

        let split_line = first_line.trim().split(",");
        let parsed_digits = split_line.map(|c| c.parse::<usize>().expect("whatever"));

        parsed_digits.collect()
    }

    fn get_game_boards(file_name: &str) -> BingoBoardCollection {
        let input_file = File::open(file_name).expect("I guess I couldn't open that file.");

        let reader = BufReader::new(input_file);

        let mut result = Vec::new();

        let mut line_counter = 0;

        let mut game_field_values = Vec::<usize>::new();

        for line in reader.lines() {
            if line_counter < 2 {
                line_counter += 1;
                continue;
            }

            let line_val = line.unwrap();

            let mut values = line_val
                .split(" ")
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            game_field_values.append(&mut values);

            line_counter += 1;
        }

        let game_board_count = game_field_values.len() / 25;
        let mut board_index = 0;

        while board_index < game_board_count {
            let start = board_index * 25;
            let end = start + 25;

            result.push(BingoBoard::new(
                game_field_values[start..end]
                    .try_into()
                    .expect("Guess I couldn't create an array like that..."),
            ));

            board_index += 1;
        }

        BingoBoardCollection::new(result)
    }
}
