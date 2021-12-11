use crate::position::Position;
use std::convert::TryFrom;

struct Octopus {
    energy_level: u32,
    flashed: bool,
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

fn build_candidates(height: usize, width: usize) -> Vec<Position> {
    (0..height).fold(vec![], |mut locs, y| {
        let mut row = (0..width).into_iter().map(|x| Position { x, y }).collect();
        locs.append(&mut row);
        locs
    })
}

fn calculate_flashes(input: Vec<String>, steps: usize) -> usize {
    let mut octopi: Vec<Vec<Octopus>> = input
        .into_iter()
        .map(|line| line.chars().filter_map(|c| c.try_into().ok()).collect())
        .collect();

    let height = octopi.len();
    let width = octopi.get(0).map(|row| row.len()).unwrap_or_default();

    (0..steps).fold(0, |mut flashes, _step| {
        let mut flashed = false;

        // increase energy level by one
        for row in 0..height {
            for col in 0..width {
                octopi[row][col].energy_level += 1;

                if octopi[row][col].energy_level > 9 {
                    flashed = true;
                }
            }
        }

        while flashed {
            flashed = false;

            for row in 0..height {
                for col in 0..width {
                    if octopi[row][col].energy_level > 9 && !octopi[row][col].flashed {
                        octopi[row][col].flashed = true;
                        flashes += 1;
                        flashed = true;

                        Position { x: col, y: row }
                            .neighbours_all()
                            .into_iter()
                            .filter(|position| position.x < width && position.y < height)
                            .for_each(|position| {
                                octopi[position.y][position.x].energy_level += 1;
                            });
                    }
                }
            }
        }

        for row in 0..height {
            for col in 0..width {
                if octopi[row][col].flashed {
                    octopi[row][col].energy_level = 0;
                    octopi[row][col].flashed = false;
                }
            }
        }

        flashes
    })
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

    // #[test]
    // fn test_calculate_incomplete_score_with_example_input() {
    //     let input = use_example_input();
    //     let expected = 0;
    //     let actual = super::calculate_incomplete_score(input);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_calculate_incomplete_score_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 0;
    //     let actual = super::calculate_incomplete_score(input);

    //     assert_eq!(expected, actual);
    // }
}
