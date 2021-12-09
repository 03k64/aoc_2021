use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct PatternMap {
    mappings: HashMap<String, char>,
}

impl PatternMap {
    fn get(&self, key: &HashSet<char>) -> Option<char> {
        let key = Self::normalise_key(key);
        self.mappings.get(&key).map(|c| *c)
    }

    fn insert(&mut self, key: &HashSet<char>, value: char) {
        let key = Self::normalise_key(key);
        self.mappings.insert(key, value);
    }

    fn normalise_key(key: &HashSet<char>) -> String {
        let mut chars: Vec<char> = key.iter().map(|c| *c).collect();
        chars.sort();
        chars.into_iter().collect()
    }
}

fn find_and_remove_by_len(sets: &mut Vec<HashSet<char>>, target_len: usize) -> HashSet<char> {
    let value_ix = sets
        .iter()
        .position(|set| set.len() == target_len)
        .unwrap()
        .clone();

    sets.remove(value_ix)
}

fn find_and_remove_by_len_and_differences(
    sets: &mut Vec<HashSet<char>>,
    len: usize,
    other: &HashSet<char>,
    differences: usize,
) -> HashSet<char> {
    let value_ix = sets
        .iter()
        .position(|set| set.len() == len && set.difference(other).count() == differences)
        .unwrap();

    sets.remove(value_ix)
}

fn find_and_remove_unique_patterns(sets: &mut Vec<HashSet<char>>) -> [HashSet<char>; 4] {
    [2, 3, 4, 7].map(|len| find_and_remove_by_len(sets, len))
}

fn segments_to_sets(segments: &str) -> Vec<HashSet<char>> {
    segments
        .split_whitespace()
        .map(|segment| HashSet::from_iter(segment.chars()))
        .collect()
}

fn count_unique_digits(input: Vec<String>) -> usize {
    input
        .into_iter()
        .filter_map(|line| {
            let (_, displayed_values) = line.split_once(" | ")?;
            displayed_values
                .split_whitespace()
                .filter(|f| f.len() == 2 || f.len() == 3 || f.len() == 4 || f.len() == 7)
                .count()
                .into()
        })
        .sum()
}

fn sum_displayed_digits(input: Vec<String>) -> usize {
    input.into_iter().fold(0, |sum, line| {
        let mut set_mappings = PatternMap::default();

        let (signal_patterns, display_values) = line.split_once(" | ").unwrap();
        let mut signal_patterns = segments_to_sets(signal_patterns);
        let display_values = segments_to_sets(display_values);

        let [one, seven, four, eight] = find_and_remove_unique_patterns(&mut signal_patterns);
        set_mappings.insert(&one, '1');
        set_mappings.insert(&seven, '7');
        set_mappings.insert(&four, '4');
        set_mappings.insert(&eight, '8');

        let filter = HashSet::from_iter(seven.union(&four).into_iter().collect::<String>().chars());
        let nine = find_and_remove_by_len_and_differences(&mut signal_patterns, 6, &filter, 1);
        set_mappings.insert(&nine, '9');

        let three_ix = signal_patterns
            .iter()
            .position(|set| set.len() == 5 && one.difference(&set).count() == 0)
            .unwrap();

        let three = signal_patterns.remove(three_ix);
        set_mappings.insert(&three, '3');

        let two = find_and_remove_by_len_and_differences(&mut signal_patterns, 5, &nine, 1);
        set_mappings.insert(&two, '2');

        let five = find_and_remove_by_len(&mut signal_patterns, 5);
        set_mappings.insert(&five, '5');

        let six_ix = signal_patterns
            .iter()
            .position(|set| five.difference(&set).count() == 0)
            .unwrap();

        let six = signal_patterns.remove(six_ix);
        set_mappings.insert(&six, '6');

        let zero = signal_patterns.remove(0);
        set_mappings.insert(&zero, '0');

        let display_value: usize = display_values
            .iter()
            .map(|set| set_mappings.get(set).unwrap())
            .collect::<String>()
            .parse()
            .unwrap_or_default();

        sum + display_value
    })
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_eight.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_count_unique_digits_with_example_input() {
        let input = use_example_input();
        let expected = 26;
        let actual = super::count_unique_digits(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_unique_digits_with_real_input() {
        let input = use_real_input();
        let expected = 383;
        let actual = super::count_unique_digits(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sum_displayed_digits_with_example_input() {
        let input = use_example_input();
        let expected = 61229;
        let actual = super::sum_displayed_digits(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sum_displayed_digits_with_real_input() {
        let input = use_real_input();
        let expected = 998900;
        let actual = super::sum_displayed_digits(input);

        assert_eq!(expected, actual);
    }
}
