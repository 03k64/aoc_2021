use std::{collections::HashMap, hash::Hash, str::FromStr};

trait Point: Sized {
    fn calculate_scalar(&self, other: Self) -> Option<Vec<Self>>;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct OrthogonalPoint {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for OrthogonalPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl FromStr for OrthogonalPoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap_or_default();

        match (x.parse(), y.parse()) {
            (Ok(x), Ok(y)) => Ok(Self::from((x, y))),
            _ => Err(()),
        }
    }
}

impl Point for OrthogonalPoint {
    fn calculate_scalar(&self, other: Self) -> Option<Vec<Self>> {
        match (self.x == other.x, self.y == other.y) {
            (true, true) => Some(vec![*self]),
            (true, false) => Some(self.vertical_scalar(other)),
            (false, true) => Some(self.horizontal_scalar(other)),
            (false, false) => None,
        }
    }
}

impl OrthogonalPoint {
    fn horizontal_scalar(&self, other: Self) -> Vec<Self> {
        if self.x > other.x {
            (other.x..=self.x).map(|x| Self { x, y: self.y }).collect()
        } else {
            (self.x..=other.x).map(|x| Self { x, y: self.y }).collect()
        }
    }

    fn vertical_scalar(&self, other: Self) -> Vec<Self> {
        if self.y > other.y {
            (other.y..=self.y).map(|y| Self { y, x: self.x }).collect()
        } else {
            (self.y..=other.y).map(|y| Self { y, x: self.x }).collect()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DiagonalPoint {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for DiagonalPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl FromStr for DiagonalPoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap_or_default();

        match (x.parse(), y.parse()) {
            (Ok(x), Ok(y)) => Ok(Self::from((x, y))),
            _ => Err(()),
        }
    }
}

impl Point for DiagonalPoint {
    fn calculate_scalar(&self, _other: Self) -> Option<Vec<Self>> {
        None
    }
}

fn calculate_overlapping_points<P>(input: Vec<String>) -> usize where P: Point + Eq + FromStr + Hash {
    input
        .iter()
        .fold(
            HashMap::with_capacity(input.len()),
            |mut frequencies, line| {
                let (a, b) = line.split_once(" -> ").unwrap_or_default();

                match (a.parse::<P>(), b.parse()) {
                    (Ok(a), Ok(b)) => {
                        let scalar = a.calculate_scalar(b).unwrap_or_default();
                        scalar.into_iter().for_each(|point| {
                            let frequency = frequencies.entry(point).or_insert(0);
                            *frequency += 1;
                        });
                    }
                    _ => {}
                }

                frequencies
            },
        )
        .values()
        .filter(|frequency| **frequency > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day_five::{DiagonalPoint, OrthogonalPoint};

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
    fn test_calculate_overlapping_orthogonal_points_with_example_input() {
        let input = use_example_input();
        let expected = 5;
        let actual = super::calculate_overlapping_points::<OrthogonalPoint>(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_overlapping_orthogonal_points_with_real_input() {
        let input = use_real_input();
        let expected = 5835;
        let actual = super::calculate_overlapping_points::<OrthogonalPoint>(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_overlapping_diagonal_points_with_example_input() {
        let input = use_example_input();
        let expected = 12;
        let actual = super::calculate_overlapping_points::<DiagonalPoint>(input);

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_calculate_overlapping_diagonal_points_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 10478;
    //     let actual = super::calculate_overlapping_points(input);

    //     assert_eq!(expected, actual);
    // }
}
