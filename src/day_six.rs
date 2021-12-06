fn model_population(seed: String, duration: usize) -> usize {
    let mut population: Vec<usize> = seed.split(',').filter_map(|timer| timer.parse().ok()).fold(
        vec![0; 9],
        |mut summary, fish: usize| {
            summary[fish] += 1;
            summary
        },
    );

    (0..duration).for_each(|_day| {
        population.rotate_left(1);
        population[6] += population[8];
    });

    population.iter().sum()
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> String {
        String::from("3,4,3,1,2")
    }

    fn use_real_input() -> String {
        include_str!("../input/day_six.txt").trim().to_owned()
    }

    #[test]
    fn test_model_population_with_example_input() {
        let input = use_example_input();
        let expected = 5934;
        let actual = super::model_population(input, 80);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_model_population_with_real_input() {
        let input = use_real_input();
        let expected = 390011;
        let actual = super::model_population(input, 80);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_model_population_with_example_input_and_longer_duration() {
        let input = use_example_input();
        let expected = 26984457539;
        let actual = super::model_population(input, 256);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_model_population_with_real_input_and_longer_lifespan() {
        let input = use_real_input();
        let expected = 1746710169834;
        let actual = super::model_population(input, 256);

        assert_eq!(expected, actual);
    }
}
