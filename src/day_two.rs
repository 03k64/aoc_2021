use std::{ops::Add, str::FromStr};
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
enum Command {
    Down(isize),
    Forward(isize),
    Up(isize),
}

#[derive(Clone, Debug, Error)]
enum CommandError {
    #[error(
        "the command `{0}` is invalid, expected `down <units>`, `forward <units>` or `up <units>`"
    )]
    InvalidCommand(String),
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tag, units) = s.split_once(' ')
            .ok_or(CommandError::InvalidCommand(s.to_owned()))?;

        let units = units
            .parse()
            .map_err(|_| CommandError::InvalidCommand(s.to_owned()))?;

        match tag {
            "down" => Ok(Command::Down(units)),
            "forward" => Ok(Command::Forward(units)),
            "up" => Ok(Command::Up(units)),
            _ => Err(CommandError::InvalidCommand(s.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct SimplePosition {
    depth: isize,
    horizontal: isize,
}

impl Add<Command> for SimplePosition {
    type Output = Self;

    fn add(self, command: Command) -> Self::Output {
        match command {
            Command::Down(units) => Self {
                horizontal: self.horizontal,
                depth: self.depth + units,
            },
            Command::Forward(units) => Self {
                horizontal: self.horizontal + units,
                depth: self.depth,
            },
            Command::Up(units) => Self {
                horizontal: self.horizontal,
                depth: self.depth - units,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct AimedPosition {
    aim: isize,
    depth: isize,
    horizontal: isize,
}

impl Add<Command> for AimedPosition {
    type Output = Self;

    fn add(self, command: Command) -> Self::Output {
        match command {
            Command::Down(units) => Self {
                aim: self.aim + units,
                horizontal: self.horizontal,
                depth: self.depth,
            },
            Command::Forward(units) => Self {
                aim: self.aim,
                horizontal: self.horizontal + units,
                depth: self.depth + self.aim * units,
            },
            Command::Up(units) => Self {
                aim: self.aim - units,
                horizontal: self.horizontal,
                depth: self.depth,
            }
        }
    }
}

fn calculate_position<P>(commands: &str) -> P where P: Default + Add<Command, Output = P> {
    commands
        .lines()
        .filter_map(|value| value.parse::<Command>().ok())
        .fold(P::default(), |position, command| position + command)
}

#[cfg(test)]
mod tests {
    use crate::{day_two::{AimedPosition, SimplePosition}, input};

    fn read_commands_from_input_file() -> String {
        input::read_input_file("day_two.txt")
            .expect("Could not read: input/day_two.txt")
    }

    #[test]
    fn test_calculate_position_with_example_input() {
        let expected = 150;
        let actual = super::calculate_position::<SimplePosition>(r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#);

        assert_eq!(expected, actual.horizontal * actual.depth);
    }

    #[test]
    fn test_calculate_position_with_real_input() {
        let commands = read_commands_from_input_file();
        let expected = 1727835;
        let actual = super::calculate_position::<SimplePosition>(&commands);

        assert_eq!(expected, actual.horizontal * actual.depth);
    }

    #[test]
    fn test_calculate_position_with_example_input_for_sliding_window_of_three() {
        let expected = 900;
        let actual = super::calculate_position::<AimedPosition>(r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#);

        assert_eq!(expected, actual.horizontal * actual.depth);
    }

    #[test]
    fn test_calculate_position_with_real_input_for_sliding_window_of_three() {
        let commands = read_commands_from_input_file();
        let expected = 1544000595;
        let actual = super::calculate_position::<AimedPosition>(&commands);

        assert_eq!(expected, actual.horizontal * actual.depth);
    }
}
