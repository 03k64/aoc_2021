fn calculate_minimum_fuel_usage(input: String) -> usize {
    let mut positions: Vec<isize> = input
        .split(',')
        .filter_map(|position| position.parse().ok())
        .collect();

    positions.sort();

    let len = positions.len();
    let median_ix = if len % 2 == 0 { len / 2 - 1 } else { len / 2 };
    let median = positions[median_ix];

    positions
        .iter()
        .map(|position| (position - median).abs() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> String {
        String::from("16,1,2,0,4,2,7,1,2,14")
    }

    fn use_real_input() -> String {
        include_str!("../input/day_seven.txt").trim().to_owned()
    }

    #[test]
    fn test_calculate_minimum_fuel_usage_with_example_input() {
        let input = use_example_input();
        let expected = 37;
        let actual = super::calculate_minimum_fuel_usage(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_minimum_fuel_usage_with_real_input() {
        let input = use_real_input();
        let expected = 0;
        let actual = super::calculate_minimum_fuel_usage(input);

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_calculate_minimum_fuel_usage_with_example_input_and_longer_duration() {
    //     let input = use_example_input();
    //     let expected = 26984457539;
    //     let actual = super::calculate_minimum_fuel_usage(input, 256);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_calculate_minimum_fuel_usage_with_real_input_and_longer_lifespan() {
    //     let input = use_real_input();
    //     let expected = 1746710169834;
    //     let actual = super::calculate_minimum_fuel_usage(input, 256);

    //     assert_eq!(expected, actual);
    // }
}
