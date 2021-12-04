fn filter_candidates(candidates: Vec<String>, prefix: &str) -> Vec<String> {
    candidates
        .into_iter()
        .filter(|c| c.starts_with(prefix))
        .collect()
}

fn is_column_one(column: usize) -> impl FnMut(&&String) -> bool {
    move |diagnostic: &&String| diagnostic.get(column..column + 1).unwrap_or_default() == "1"
}

fn majority_one(candidates: &Vec<String>, column: usize, threshold: usize, allow_eq: bool) -> bool {
    let frequency = candidates.iter().filter(is_column_one(column)).count();

    if allow_eq {
        frequency >= threshold
    } else {
        frequency > threshold
    }
}

fn calculate_power_consumption(diagnostics: &Vec<String>) -> u64 {
    let threshold = diagnostics.len() / 2;
    let width = diagnostics.get(0).map(|d| d.len()).unwrap_or_default();
    let shift = 64 - width;

    let most_common_value =
        move |g, c| (g << 1) + u64::from(majority_one(diagnostics, c, threshold, false));

    let gamma = (0..width).fold(0, most_common_value);
    let epsilon = (!(gamma << shift)) >> shift;

    gamma * epsilon
}

fn calculate_life_support_rating(diagnostics: &Vec<String>) -> u64 {
    let width = diagnostics.get(0).map(|d| d.len()).unwrap_or_default();

    let first_o2_digit = u64::from(majority_one(
        &diagnostics,
        0,
        (diagnostics.len() + 1) / 2,
        true,
    ));

    let mut co2 = (1 - first_o2_digit).to_string();
    co2.reserve(width - 1);

    let mut o2 = first_o2_digit.to_string();
    o2.reserve(width - 1);

    let mut o2_candidates: Vec<String> = filter_candidates(diagnostics.clone(), &o2);
    let mut o2_threshold = (o2_candidates.len() + 1) / 2;

    let mut co2_candidates: Vec<String> = filter_candidates(diagnostics.clone(), &co2);
    let mut co2_threshold = (co2_candidates.len() + 1) / 2;

    for col in 1..width {
        if o2_candidates.len() > 1 {
            let digit = u64::from(majority_one(&o2_candidates, col, o2_threshold, true));
            o2.push_str(digit.to_string().as_str());
            o2_candidates = filter_candidates(o2_candidates, &o2);
            o2_threshold = (o2_candidates.len() + 1) / 2;
        } else if o2_candidates.len() == 1 {
            o2 = o2_candidates.remove(0);
        }

        if co2_candidates.len() > 1 {
            let digit = u64::from(!majority_one(&co2_candidates, col, co2_threshold, true));
            co2.push_str(digit.to_string().as_str());
            co2_candidates = filter_candidates(co2_candidates, &co2);
            co2_threshold = (co2_candidates.len() + 1) / 2;
        } else if co2_candidates.len() == 1 {
            co2 = co2_candidates.remove(0);
        }
    }

    u64::from_str_radix(&o2, 2).unwrap_or_default()
        * u64::from_str_radix(&co2, 2).unwrap_or_default()
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

    #[test]
    fn test_calculate_life_support_rating_with_example_input_for_sliding_window_of_three() {
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

        let expected = 230;
        let actual = super::calculate_life_support_rating(&diagnostics);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_life_support_rating_with_real_input_for_sliding_window_of_three() {
        let diagnostics = read_diagnostics_from_input_file();
        let expected = 2784375;
        let actual = super::calculate_life_support_rating(&diagnostics);

        assert_eq!(expected, actual);
    }
}
