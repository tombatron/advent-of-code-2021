pub fn simulate_fish(days: usize, initial_state: &mut Vec<usize>) -> usize {
    let mut current_day = 0;

    let current_state = initial_state;

    while current_day < days {
        println!("Working on day {}", (current_day + 1));

        //let mut next_state = Vec::new();
        let mut new_fish = Vec::new();

        for fish in &mut *current_state {
            if let 1.. = fish {
                //next_state.push(fish - 1)
                *fish -= 1;
            } else {
                *fish = 6;

                new_fish.push(8);
            }
        }

        current_state.extend(new_fish);

        current_day += 1;
    }

    (&current_state).len()
}

#[cfg(test)]
mod tests {
    use crate::aoc_utils::get_external_input;

    use super::simulate_fish;

    #[test]
    fn part_one_sample_input_works() {
        let mut sample_input = vec![3, 4, 3, 1, 2];

        let result = simulate_fish(80, &mut sample_input);

        assert_eq!(5934, result);
    }

    #[test]
    fn part_one_external_input_work() {
        let mut input = get_test_input();

        let result = simulate_fish(80, &mut input);

        assert_eq!(353274, result);
    }

    #[test]
    fn part_two_sample_input_works() {
        let mut sample_input = vec![3, 4, 3, 1, 2];

        let result = simulate_fish(256, &mut sample_input);

        assert_eq!(26984457539, result);
    }    

    fn get_test_input() -> Vec<usize> {
        let test_input = get_external_input("day_06_input.txt", parse_line);

        test_input.first().unwrap().to_vec()
    }

    fn parse_line(line: String) -> Vec<usize> {
        line
            .split(",")
            .into_iter()
            .map(|f| f.trim())
            .map(|f| f.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }
}
