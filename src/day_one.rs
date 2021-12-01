fn count_increases_raw(measurements: &Vec<usize>) -> usize {
    (1..measurements.len())
        .filter(|ix| measurements[*ix] > measurements[ix - 1])
        .count()
}

fn count_increases_sliding_window(measurements: &Vec<usize>, window_size: usize) -> usize {
    let window_sums = measurements
        .windows(window_size)
        .map(|window| window.iter().sum())
        .collect();

    count_increases_raw(&window_sums)
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
    fn test_count_increases_raw_with_example_input() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;
        let actual = super::count_increases_raw(&measurements);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_increases_raw_with_real_input() {
        let measurements = read_measurements_from_input_file();
        let expected = 1266;
        let actual = super::count_increases_raw(&measurements);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_increases_sliding_window_with_example_input() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;
        let actual = super::count_increases_sliding_window(&measurements, WINDOW_SIZE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_increases_sliding_window_with_real_input() {
        let measurements = read_measurements_from_input_file();
        let expected = 1217;
        let actual = super::count_increases_sliding_window(&measurements, WINDOW_SIZE);

        assert_eq!(expected, actual);
    }
}
