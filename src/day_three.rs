// turn each column into a usize (usize::from_str_radix(i, 2))
// use usize.count_ones() > column.len() / 2 to determine bit in new number
fn calculate_power_consumption(diagnostics: &Vec<String>) -> usize {
    let digits_len = diagnostics.get(0).map(|d| d.len()).unwrap_or_default();
    let mut digits = vec![0; digits_len];

    for diagnostic in diagnostics.iter() {
        for (ix, c) in diagnostic.chars().enumerate() {
            let digit = c.to_digit(2)
                .map(|d| if d == 1 { 1 } else { -1 })
                .unwrap();

            digits[ix] += digit;
        }
    }

    let epsilon: String = digits.iter().map(|d| if *d < 0 { "1" } else { "0" }).collect();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    let gamma: String = digits.iter().map(|d| if *d < 0 { "0" } else { "1" }).collect();
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    fn read_diagnostics_from_input_file() -> Vec<String> {
        include_str!("../input/day_three.txt")
            .to_owned()
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_calculate_power_consumption_with_example_input() {
        let diagnostics = vec![
            String::from("00100"), String::from("11110"), String::from("10110"),
            String::from("10111"), String::from("10101"), String::from("01111"),
            String::from("00111"), String::from("11100"), String::from("10000"),
            String::from("11001"), String::from("00010"), String::from("01010"),
        ];

        let expected = 198;
        let actual = super::calculate_power_consumption(&diagnostics);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_power_consumption_with_real_input() {
        let diagnostics = read_diagnostics_from_input_file();
        let expected = 2583164;
        let actual = super::calculate_power_consumption(&diagnostics);

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_calculate_power_consumption_with_example_input_for_sliding_window_of_three() {
    //     let diagnostics = vec![
    //         String::from("00100"), String::from("11110"), String::from("10110"),
    //         String::from("10111"), String::from("10101"), String::from("01111"),
    //         String::from("00111"), String::from("11100"), String::from("10000"),
    //         String::from("11001"), String::from("00010"), String::from("01010"),
    //     ];

    //     let expected = 0;
    //     let actual = super::calculate_power_consumption(&diagnostics);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_calculate_power_consumption_with_real_input_for_sliding_window_of_three() {
    //     let diagnostics = read_diagnostics_from_input_file();
    //     let expected = 1544000595;
    //     let actual = super::calculate_power_consumption(&diagnostics);

    //     assert_eq!(expected, actual);
    // }
}
