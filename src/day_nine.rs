fn sum_risk_levels(input: Vec<String>) -> u32 {
    let height = input.len();
    let width = input.get(0).map(|row| row.len()).unwrap_or_default();

    let mut matrix = vec![vec![0u32; width]; height];
    for (rix, row) in input.iter().enumerate() {
        for (cix, col) in row.chars().enumerate() {
            matrix[rix][cix] = col.to_digit(10).unwrap();
        }
    }

    let mut low_points = vec![];
    for (rix, _) in matrix.iter().enumerate() {
        for (cix, col) in matrix[rix].iter().enumerate() {
            let top = rix == 0 || (rix > 0 && *col < matrix[rix - 1][cix]);
            let bottom = rix == height - 1 || (rix < height - 1 && *col < matrix[rix + 1][cix]);
            let left = cix == 0 || (cix > 0 && *col < matrix[rix][cix - 1]);
            let right = cix == width - 1 || (cix < width - 1 && *col < matrix[rix][cix + 1]);

            if top && bottom && left && right {
                low_points.push(*col);
            }
        }
    }

    low_points.into_iter().map(|lp| lp + 1).sum()
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"2199943210
3987894921
9856789892
8767896789
9899965678"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_nine.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_sum_risk_levels_with_example_input() {
        let input = use_example_input();
        let expected = 15;
        let actual = super::sum_risk_levels(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sum_risk_levels_with_real_input() {
        let input = use_real_input();
        let expected = 580;
        let actual = super::sum_risk_levels(input);

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_sum_displayed_digits_with_example_input() {
    //     let input = use_example_input();
    //     let expected = 61229;
    //     let actual = super::sum_displayed_digits(input);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_sum_displayed_digits_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 998900;
    //     let actual = super::sum_displayed_digits(input);

    //     assert_eq!(expected, actual);
    // }
}
