use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Lanternfish {
    timer: u8,
}

impl FromStr for Lanternfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timer = s.parse().map_err(|_| ())?;

        if timer <= 8 {
            Ok(Self { timer })
        } else {
            Err(())
        }
    }
}

impl Lanternfish {
    fn new() -> Self {
        Self { timer: 8 }
    }

    fn age(&mut self) -> Option<Self> {
        if self.timer == 0 {
            self.timer = 6;
            Some(Self::new())
        } else {
            self.timer -= 1;
            None
        }
    }
}

fn model_population(seed: String, duration: u8) -> usize {
    let mut population: Vec<Lanternfish> = seed
        .split(',')
        .filter_map(|timer| timer.parse().ok())
        .collect();

    (0..duration)
        .for_each(|day| {
            let mut new_fish: Vec<Lanternfish> = population.iter_mut().filter_map(|f| f.age()).collect();
            dbg!(day, new_fish.len());
            population.append(&mut new_fish);
            // dbg!(&population.iter().map(|fish| fish.timer.to_string()).collect::<String>());
        });

    population.len()
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> String {
        String::from("3,4,3,1,2")
    }

    fn use_real_input() -> String {
        include_str!("../input/day_six.txt").to_owned()
    }

    // #[test]
    // fn test_model_population_with_example_input() {
    //     let input = use_example_input();
    //     let expected = 5934;
    //     let actual = super::model_population(input, 80);

    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn test_model_population_with_real_input() {
        let input = use_real_input();
        let expected = 0;
        let actual = super::model_population(input, 80);

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_model_population_with_example_input() {
    //     let input = use_example_input();
    //     let expected = 12;
    //     let actual = super::model_population(input);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_model_population_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 17013;
    //     let actual = super::model_population(input);

    //     assert_eq!(expected, actual);
    // }
}
