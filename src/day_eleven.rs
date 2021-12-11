use crate::position::Position;
use std::{convert::TryFrom, ops::Add};

#[derive(Clone, Copy, Default)]
struct Octopus {
    energy_level: u32,
    flashed: bool,
}

impl Add<u32> for Octopus {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            energy_level: self.energy_level + rhs,
            flashed: self.flashed,
        }
    }
}

impl TryFrom<char> for Octopus {
    type Error = ();

    fn try_from(energy_level: char) -> Result<Self, Self::Error> {
        if let Some(energy_level) = energy_level.to_digit(10) {
            Ok(Self {
                flashed: false,
                energy_level,
            })
        } else {
            Err(())
        }
    }
}

impl Octopus {
    fn reset_if_flashed(self) -> Self {
        if self.flashed {
            Self::default()
        } else {
            self
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Octopus> {
    input
        .into_iter()
        .flat_map(|line| {
            line.chars()
                .filter_map(|c| c.try_into().ok())
                .collect::<Vec<Octopus>>()
        })
        .collect()
}

pub fn calculate_flashes(input: Vec<String>, steps: usize) -> usize {
    let height = input.len();
    let width = input
        .get(0)
        .map(|row| row.chars().count())
        .unwrap_or_default();

    let (flashes, _) = (0..steps).fold(
        (0, parse_input(input)),
        |(mut flashes, mut octopi), _step| {
            let mut flashed = true;

            octopi = octopi.into_iter().map(|o| o + 1).collect();

            while flashed {
                flashed = false;

                for ix in 0..octopi.len() {
                    if octopi[ix].energy_level > 9 && !octopi[ix].flashed {
                        octopi[ix].flashed = true;
                        flashes += 1;
                        flashed = true;

                        Position {
                            x: ix % width,
                            y: ix / height,
                        }
                        .neighbours_all(height, width)
                        .into_iter()
                        .for_each(|position| {
                            octopi[position.y * height + position.x].energy_level += 1;
                        });
                    }
                }
            }

            octopi = octopi.into_iter().map(|o| o.reset_if_flashed()).collect();

            (flashes, octopi)
        },
    );

    flashes
}

pub fn find_synchronised_flash(input: Vec<String>) -> usize {
    let height = input.len();
    let width = input
        .get(0)
        .map(|row| row.chars().count())
        .unwrap_or_default();

    let mut octopi = parse_input(input);

    (1..usize::MAX)
        .find(|_step| {
            let mut flashed = true;
            let mut flashes = 0;

            for ix in 0..octopi.len() {
                octopi[ix] = octopi[ix] + 1;
            }

            while flashed {
                flashed = false;

                for ix in 0..octopi.len() {
                    if octopi[ix].energy_level > 9 && !octopi[ix].flashed {
                        octopi[ix].flashed = true;
                        flashes += 1;
                        flashed = true;

                        Position {
                            x: ix % width,
                            y: ix / height,
                        }
                        .neighbours_all(height, width)
                        .into_iter()
                        .for_each(|position| {
                            octopi[position.y * height + position.x].energy_level += 1;
                        });
                    }
                }
            }

            for ix in 0..octopi.len() {
                octopi[ix] = octopi[ix].reset_if_flashed();
            }

            flashes == octopi.len()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_eleven.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_calculate_flashes_with_example_input() {
        let input = use_example_input();
        let expected = 1656;
        let actual = super::calculate_flashes(input, 100);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_flashes_with_real_input() {
        let input = use_real_input();
        let expected = 1757;
        let actual = super::calculate_flashes(input, 100);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_synchronised_flash_with_example_input() {
        let input = use_example_input();
        let expected = 195;
        let actual = super::find_synchronised_flash(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_synchronised_flash_with_real_input() {
        let input = use_real_input();
        let expected = 422;
        let actual = super::find_synchronised_flash(input);

        assert_eq!(expected, actual);
    }
}
