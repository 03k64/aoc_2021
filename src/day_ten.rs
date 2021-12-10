fn calculate_corrupted_score(input: Vec<String>) -> usize {
    input.into_iter().fold(0, |sum, line| {
        let mut stack = vec![];
        for symbol in line.chars() {
            match symbol {
                '(' | '[' | '{' | '<' => stack.push(symbol),
                ')' => {
                    if stack.pop() != Some('(') {
                        return sum + 3;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return sum + 57;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return sum + 1197;
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        return sum + 25137;
                    }
                }
                _ => {}
            }
        }

        sum
    })
}

fn calculate_incomplete_score(input: Vec<String>) -> usize {
    let mut scores: Vec<usize> = input
        .into_iter()
        .filter_map(|line| {
            let mut stack = vec![];
            for symbol in line.chars() {
                match symbol {
                    '(' | '[' | '{' | '<' => stack.push(symbol),
                    ')' => {
                        if stack.pop() != Some('(') {
                            return None;
                        }
                    }
                    ']' => {
                        if stack.pop() != Some('[') {
                            return None;
                        }
                    }
                    '}' => {
                        if stack.pop() != Some('{') {
                            return None;
                        }
                    }
                    '>' => {
                        if stack.pop() != Some('<') {
                            return None;
                        }
                    }
                    _ => {}
                };
            }

            stack
                .into_iter()
                .rev()
                .fold(0, |total, symbol| match symbol {
                    '(' => total * 5 + 1,
                    '[' => total * 5 + 2,
                    '{' => total * 5 + 3,
                    '<' => total * 5 + 4,
                    _ => total,
                })
                .into()
        })
        .collect();

    scores.sort();

    if scores.len() % 2 == 0 {
        scores[scores.len() / 2]
    } else {
        scores[(scores.len() - 1) / 2]
    }
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_ten.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_calculate_corrupted_score_with_example_input() {
        let input = use_example_input();
        let expected = 26397;
        let actual = super::calculate_corrupted_score(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_corrupted_score_with_real_input() {
        let input = use_real_input();
        let expected = 358737;
        let actual = super::calculate_corrupted_score(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_incomplete_score_with_example_input() {
        let input = use_example_input();
        let expected = 288957;
        let actual = super::calculate_incomplete_score(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_incomplete_score_with_real_input() {
        let input = use_real_input();
        let expected = 4329504793;
        let actual = super::calculate_incomplete_score(input);

        assert_eq!(expected, actual);
    }
}
