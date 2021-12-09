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

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#).lines().map(|s| s.to_owned()).collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_eight.txt")
            .lines()
            .map(|s| s.to_owned())
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

    // #[test]
    // fn test_determine_displayed_value_with_example_input() {
    //     let input = use_example_input();
    //     let expected = 168;
    //     let actual = super::determine_displayed_value(input);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_determine_displayed_value_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 104149091;
    //     let actual = super::determine_displayed_value(input);

    //     assert_eq!(expected, actual);
    // }
}
