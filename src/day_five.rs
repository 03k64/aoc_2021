use std::{cmp::Ordering, collections::HashMap, hash::Hash};

trait Point: Copy + Eq + PartialEq + Sized {
    fn new(x: usize, y: usize) -> Self;

    fn x(&self) -> usize;

    fn y(&self) -> usize;

    fn from_coords(coords: &str) -> Result<Self, ()> {
        let (x, y) = coords.split_once(',').unwrap_or_default();

        match (x.parse(), y.parse()) {
            (Ok(x), Ok(y)) => Ok(Self::new(x, y)),
            _ => Err(()),
        }
    }

    fn calculate_scalar(self, other: Self) -> Option<Vec<Self>> {
        match (self.x().cmp(&other.x()), self.y().cmp(&other.y())) {
            (Ordering::Equal, Ordering::Equal) => Some(vec![self]),
            (Ordering::Equal, Ordering::Greater | Ordering::Less) => Some(vertical_scalar(self, other)),
            (Ordering::Greater | Ordering::Less, Ordering::Equal) => Some(horizontal_scalar(self, other)),
            _ => Some(diagonal_scalar(self, other)),
        }
    }
}

fn diagonal_scalar<P>(a: P, b: P) -> Vec<P> where P: Point {
    let mut current = a;
    let mut scalar = vec![];

    let x_step: isize = match a.x().cmp(&b.x()) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    };

    let y_step: isize = match a.y().cmp(&b.y()) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    };

    while current != b {
        let next_x = current.x() as isize + x_step;
        let next_y = current.y() as isize + y_step;
        scalar.push(current);

        current = P::new(next_x as usize, next_y as usize);
    }

    scalar.push(current);

    scalar
}

fn horizontal_scalar<P>(a: P, b: P) -> Vec<P> where P: Point {
    if a.x() > b.x() {
        (b.x()..=a.x()).map(|x| P::new(x, a.y())).collect()
    } else {
        (a.x()..=b.x()).map(|x| P::new(x, a.y())).collect()
    }
}

fn vertical_scalar<P>(a: P, b: P) -> Vec<P> where P: Point {
    if a.y() > b.y() {
        (b.y()..=a.y()).map(|y| P::new(a.x(), y)).collect()
    } else {
        (a.y()..=b.y()).map(|y| P::new(a.x(), y)).collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct OrthogonalPoint {
    x: usize,
    y: usize,
}

impl Point for OrthogonalPoint {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn calculate_scalar(self, other: Self) -> Option<Vec<Self>> {
        match (self.x().cmp(&other.x()), self.y().cmp(&other.y())) {
            (Ordering::Equal, Ordering::Equal) => Some(vec![self]),
            (Ordering::Equal, Ordering::Greater | Ordering::Less) => Some(vertical_scalar(self, other)),
            (Ordering::Greater | Ordering::Less, Ordering::Equal) => Some(horizontal_scalar(self, other)),
            _ => None, // effectively remove diagonal scalar implementation for orthogonal point
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DiagonalPoint {
    x: usize,
    y: usize,
}

impl Point for DiagonalPoint {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

fn calculate_overlapping_points<P>(input: Vec<String>) -> usize where P: Point + Eq + Hash {
    input
        .iter()
        .fold(
            HashMap::with_capacity(input.len()),
            |mut frequencies, line| {
                let (a, b) = line.split_once(" -> ").unwrap_or_default();

                match (P::from_coords(a), P::from_coords(b)) {
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

    #[test]
    fn test_calculate_overlapping_diagonal_points_with_real_input() {
        let input = use_real_input();
        let expected = 17013;
        let actual = super::calculate_overlapping_points::<DiagonalPoint>(input);

        assert_eq!(expected, actual);
    }
}
