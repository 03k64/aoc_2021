/// Returns the frequency with which the sum of a sliding window is an increase on the sum of the
/// previous sliding window. To return the frequency with which an individual measurement is an
/// increase on the previous measurement, set `window_size` to `1`.
///
/// For successive sliding windows, A and B, an increase may be detected if the final element of B
/// is greater than the first element of A, all remaining elements are common to both A and B.
fn count_increases(measurements: &Vec<usize>, window_size: usize) -> usize {
    measurements
        .windows(window_size + 1)
        .filter(|window| window[window_size] > window[0])
        .count()
}

#[cfg(test)]
mod tests {
    use crate::input;

    const WINDOW_SIZE: usize = 3;

    fn read_measurements_from_input_file() -> Vec<usize> {
        input::read_input_file("day_one.txt")
            .expect("Could not read: input/day_one.txt")
            .lines()
            .map(|line| line.parse::<usize>().expect("Measurement is invalid"))
            .collect()
    }

    #[test]
    fn test_count_increases_with_example_input_for_sliding_window_of_one() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;
        let actual = super::count_increases(&measurements, 1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_increases_with_real_input_for_sliding_window_of_one() {
        let measurements = read_measurements_from_input_file();
        let expected = 1266;
        let actual = super::count_increases(&measurements, 1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_increases_with_example_input_for_sliding_window_of_three() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;
        let actual = super::count_increases(&measurements, 3);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_increases_with_real_input_for_sliding_window_of_three() {
        let measurements = read_measurements_from_input_file();
        let expected = 1217;
        let actual = super::count_increases(&measurements, 3);

        assert_eq!(expected, actual);
    }
}
