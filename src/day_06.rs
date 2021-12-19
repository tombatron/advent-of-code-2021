pub fn simulate_fish(days: usize, initial_state: Vec<usize>) -> usize {
    let mut current_day = 0;

    let mut current_state = initial_state;

    while current_day < days {
        let mut next_state = Vec::new();

        for fish in current_state {
            if let 1.. = fish {
                next_state.push(fish - 1)
            } else {
                next_state.push(6);
                next_state.push(8);
            }
        }

        current_state = next_state;

        current_day += 1;
    }

    current_state.len()
}

#[cfg(test)]
mod tests {
    use super::simulate_fish;

    #[test]
    fn part_one_sample_input_works() {
        let sample_input = vec![3, 4, 3, 1, 2];

        let result = simulate_fish(80, sample_input);

        assert_eq!(5934, result);
    }
}
