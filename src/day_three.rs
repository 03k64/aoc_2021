fn transpose(matrix: &Vec<String>) -> Vec<String> {
    let height = matrix.len();
    let width = matrix.get(0).map(|d| d.len()).unwrap_or_default();

    let mut transposed = vec![String::with_capacity(height); width];
    for row in matrix {
        let row_chars: Vec<char> = row.chars().collect();
        for i in 0..width {
            transposed[i].push(row_chars[i]);
        }
    }

    transposed
}

fn calculate_power_consumption(diagnostics: &Vec<String>) -> usize {
    let height = diagnostics.len();
    let width = diagnostics.get(0).map(|d| d.len()).unwrap_or_default();
    let threshold = height / 2;
    let transposed = transpose(diagnostics);

    let (epsilon, gamma) = transposed.into_iter().fold(
        (String::with_capacity(width), String::with_capacity(width)),
        |(mut g, mut e), t| {
            let gamma = t.replace("0", "").len() > threshold;
            let epsilon = !gamma;

            let gamma = usize::from(gamma).to_string();
            let epsilon = usize::from(epsilon).to_string();

            g.push_str(gamma.as_str());
            e.push_str(epsilon.as_str());

            (g, e)
        },
    );

    usize::from_str_radix(&epsilon, 2).unwrap_or_default()
        * usize::from_str_radix(&gamma, 2).unwrap_or_default()
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
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
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
    // fn test_calculate_life_support_rating_with_example_input_for_sliding_window_of_three() {
    //     let diagnostics = vec![
    //         String::from("00100"),
    //         String::from("11110"),
    //         String::from("10110"),
    //         String::from("10111"),
    //         String::from("10101"),
    //         String::from("01111"),
    //         String::from("00111"),
    //         String::from("11100"),
    //         String::from("10000"),
    //         String::from("11001"),
    //         String::from("00010"),
    //         String::from("01010"),
    //     ];

    //     let expected = 0;
    //     let actual = super::calculate_life_support_rating(&diagnostics);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_calculate_life_support_rating_with_real_input_for_sliding_window_of_three() {
    //     let diagnostics = read_diagnostics_from_input_file();
    //     let expected = 1544000595;
    //     let actual = super::calculate_life_support_rating(&diagnostics);

    //     assert_eq!(expected, actual);
    // }
}
