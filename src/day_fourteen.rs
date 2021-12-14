use std::collections::HashMap;

fn calculate_frequency_range(mut input: Vec<String>, steps: usize) -> usize {
    let template: Vec<char> = input.remove(0).chars().collect();
    let rules: HashMap<(char, char), char> =
        input.into_iter().fold(HashMap::new(), |mut rules, rule| {
            let (input, output) = rule.split_once(" -> ").unwrap_or_default();
            let mut input: Vec<char> = input.chars().collect();

            let left = input.remove(0);
            let right = input.remove(0);
            let output = output.parse().unwrap();
            rules.insert((left, right), output);

            rules
        });

    let mut pairs: HashMap<(char, char), usize> =
        template[..]
            .windows(2)
            .fold(HashMap::new(), |mut pairs, pair| {
                let entry = pairs.entry((pair[0], pair[1])).or_default();
                *entry += 1;
                pairs
            });

    for _ in 0..steps {
        pairs = pairs
            .into_iter()
            .fold(HashMap::new(), |mut new_pairs, ((l, r), frequency)| {
                let m = rules[&(l, r)];

                let entry = new_pairs.entry((l, m)).or_default();
                *entry += frequency;

                let entry = new_pairs.entry((m, r)).or_default();
                *entry += frequency;

                new_pairs
            });
    }

    let (max, min) = pairs
        .into_iter()
        .fold(
            HashMap::new(),
            |mut frequencies: HashMap<char, f64>, ((l, r), frequency)| {
                let entry = frequencies.entry(l).or_default();
                *entry += frequency as f64 / 2.;

                let entry = frequencies.entry(r).or_default();
                *entry += frequency as f64 / 2.;

                frequencies
            },
        )
        .into_iter()
        .map(|(_, frequency)| frequency.round() as usize)
        .fold((0, usize::MAX), |(max, min), frequency| {
            (max.max(frequency), min.min(frequency))
        });

    max - min
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#,
        )
        .lines()
        .map(String::from)
        .filter(|l| !l.is_empty())
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_fourteen.txt")
            .lines()
            .map(String::from)
            .filter(|l| !l.is_empty())
            .collect()
    }

    #[test]
    fn test_calculate_frequency_range_with_example_input_and_ten_steps() {
        let input = use_example_input();
        let expected = 1588;
        let actual = super::calculate_frequency_range(input, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_frequency_range_with_real_input_and_ten_steps() {
        let input = use_real_input();
        let expected = 3406;
        let actual = super::calculate_frequency_range(input, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_frequency_range_with_example_input_and_forty_steps() {
        let input = use_example_input();
        let expected = 2188189693529;
        let actual = super::calculate_frequency_range(input, 40);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_frequency_range_with_real_input_and_forty_steps() {
        let input = use_real_input();
        let expected = 3941782230241;
        let actual = super::calculate_frequency_range(input, 40);

        assert_eq!(expected, actual);
    }
}
