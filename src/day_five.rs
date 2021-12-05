use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq)]
struct OrthogonalPoint {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for OrthogonalPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Ord for OrthogonalPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for OrthogonalPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrthogonalPoint {
    fn calculate_scalar(&self, other: Self) -> Option<Vec<Self>> {
        match (self.x == other.x, self.y == other.y) {
            (true, true) => Some(vec![*self]),
            (true, false) => Some(self.vertical_scalar(other)),
            (false, true) => Some(self.horizontal_scalar(other)),
            (false, false) => None,
        }
    }

    fn horizontal_scalar(&self, other: Self) -> Vec<Self> {
        if self.x > other.x {
            (other.x..self.x).map(|x| Self { x, y: self.y }).collect()
        } else {
            (self.x..other.x).map(|x| Self { x, y: self.y }).collect()
        }
    }

    fn vertical_scalar(&self, other: Self) -> Vec<Self> {
        if self.y > other.y {
            (other.y..self.y).map(|y| Self { y, x: self.x }).collect()
        } else {
            (self.y..other.y).map(|y| Self { y, x: self.x }).collect()
        }
    }
}

fn calculate_overlapping_points(input: Vec<String>) -> usize {
    let points: Vec<OrthogonalPoint> = input.iter().fold(vec![], |mut points, line| {
        let (a, b) = line.split_once(" -> ").unwrap_or_default();
        let (ax, ay) = a.split_once(',').unwrap_or_default();
        let ax = ax.parse().unwrap_or_default();
        let ay = ay.parse().unwrap_or_default();
        let a = OrthogonalPoint::from((ax, ay));

        let (bx, by) = b.split_once(',').unwrap_or_default();
        let bx = bx.parse().unwrap_or_default();
        let by = by.parse().unwrap_or_default();
        let b = OrthogonalPoint::from((ax, ay));

        if let Some(mut scalar) = a.calculate_scalar(b) {
            points.append(&mut scalar);
        }

        points
    });
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_five.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_calculate_overlapping_points_with_example_input() {
        let input = use_example_input();
        let expected = 5;
        let actual = super::calculate_overlapping_points(input);

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_calculate_overlapping_points_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 41668;
    //     let actual = super::calculate_overlapping_points(input);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_calculate_overlapping_points_with_example_input() {
    //     let input = use_example_input();
    //     let expected = 1924;
    //     let actual = super::calculate_overlapping_points(input);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_calculate_overlapping_points_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 10478;
    //     let actual = super::calculate_overlapping_points(input);

    //     assert_eq!(expected, actual);
    // }
}
