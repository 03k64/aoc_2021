use std::usize;

fn parse_input(input: String) -> Vec<isize> {
    input
        .split(',')
        .filter_map(|position| position.parse().ok())
        .collect()
}

fn calculate_increasing_fuel_usage(distance: isize) -> usize {
    let distance = distance as f64;
    ((distance / 2.) * (distance + 1.)).ceil() as usize
}

fn calculate_minimum_constant_fuel_usage(input: String) -> usize {
    let mut positions = parse_input(input);
    positions.sort();

    let len = positions.len();
    let median_ix = if len % 2 == 0 { len / 2 - 1 } else { len / 2 };
    let median = positions[median_ix];

    positions
        .iter()
        .map(|position| (position - median).abs() as usize)
        .sum()
}

fn calculate_minimum_increasing_fuel_usage(input: String) -> usize {
    let positions = parse_input(input);
    let sum = positions
        .iter()
        .fold(0., |sum, position| sum + *position as f64);

    let mean = sum / positions.len() as f64;
    let lo_mean = mean.floor() as isize;
    let hi_mean = mean.ceil() as isize;

    let (lo_sum, hi_sum) = positions
        .iter()
        .fold((0, 0), |(lo_total, hi_total), position| {
            let lo = calculate_increasing_fuel_usage((position - lo_mean).abs());
            let hi = calculate_increasing_fuel_usage((position - hi_mean).abs());
            (lo_total + lo, hi_total + hi)
        });

    lo_sum.min(hi_sum)
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
    fn test_calculate_minimum_constant_fuel_usage_with_example_input() {
        let input = use_example_input();
        let expected = 37;
        let actual = super::calculate_minimum_constant_fuel_usage(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_minimum_constant_fuel_usage_with_real_input() {
        let input = use_real_input();
        let expected = 364898;
        let actual = super::calculate_minimum_constant_fuel_usage(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_minimum_increasing_fuel_usage_with_example_input() {
        let input = use_example_input();
        let expected = 168;
        let actual = super::calculate_minimum_increasing_fuel_usage(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_minimum_increasing_fuel_usage_with_real_input() {
        let input = use_real_input();
        let expected = 104149091;
        let actual = super::calculate_minimum_increasing_fuel_usage(input);

        assert_eq!(expected, actual);
    }
}
