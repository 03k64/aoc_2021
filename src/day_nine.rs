use crate::position::Position;

fn parse_input(input: Vec<String>, height: usize, width: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; width]; height];

    for (rix, row) in input.iter().enumerate() {
        for (cix, col) in row.chars().enumerate() {
            matrix[rix][cix] = col.to_digit(10).unwrap();
        }
    }

    matrix
}

fn spread(
    current: Position,
    candidates: &mut Vec<Position>,
    basin: &mut Vec<Position>,
    boundaries: &mut Vec<Position>,
    height_map: &Vec<Vec<u32>>,
) {
    if let Some(position) = candidates.iter().position(|p| *p == current) {
        // remove current position from candidates
        candidates.remove(position);
    } else {
        // if candidate has previously been removed, return
        return;
    }

    // if current position has a height value of `9`, we have reached the edge of a basin
    // add current position to boundaries and return
    if height_map[current.y][current.x] == 9 {
        boundaries.push(current);
        return;
    }

    // otherwise, must still be in the current basin, add to the basin
    basin.push(current);

    // if next position to right is still a candidate, move there
    current
        .neighbours_orthogonal()
        .into_iter()
        .for_each(|next| spread(next, candidates, basin, boundaries, height_map));
}

fn sum_risk_levels(input: Vec<String>) -> u32 {
    let height = input.len();
    let width = input.get(0).map(|row| row.len()).unwrap_or_default();
    let matrix = parse_input(input, height, width);

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

fn multiply_basin_sizes(input: Vec<String>) -> usize {
    let height = input.len();
    let width = input.get(0).map(|row| row.len()).unwrap_or_default();
    let height_map = parse_input(input, height, width);

    let mut basins = vec![];
    let mut boundaries = vec![];

    let mut candidates = (0..height).into_iter().fold(vec![], |mut locs, y| {
        let mut row = (0..width).into_iter().map(|x| Position { x, y }).collect();
        locs.append(&mut row);
        locs
    });

    while let Some(start) = candidates.get(0) {
        let mut basin = vec![];

        spread(
            *start,
            &mut candidates,
            &mut basin,
            &mut boundaries,
            &height_map,
        );

        if !basin.is_empty() {
            basins.push(basin);
        }
    }

    basins.sort_by(|a, b| a.len().cmp(&b.len()));

    basins[basins.len() - 3..]
        .iter()
        .fold(1, |product, basin| product * basin.len())
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

    #[test]
    fn test_multiply_basin_sizes_with_example_input() {
        let input = use_example_input();
        let expected = 1134;
        let actual = super::multiply_basin_sizes(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiply_basin_sizes_with_real_input() {
        let input = use_real_input();
        let expected = 856716;
        let actual = super::multiply_basin_sizes(input);

        assert_eq!(expected, actual);
    }
}
