pub fn simulate_fish(days: usize, initial_state: Vec<usize>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::simulate_fish;

    #[test]
    fn part_one_sample_input_works(){
        let sample_input = vec![3,4,3,1,2];

        let result = simulate_fish(80, sample_input);

        assert_eq!(5934, result);
    }
}