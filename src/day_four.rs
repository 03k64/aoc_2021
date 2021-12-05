use std::collections::HashMap;

#[derive(Debug)]
struct BingoCard {
    column_bingo: bool,
    columns: Vec<HashMap<usize, bool>>,
    row_bingo: bool,
    rows: Vec<HashMap<usize, bool>>,
}

impl BingoCard {
    fn new(lines: Vec<String>) -> Self {
        let matrix: Vec<Vec<usize>> = lines
            .into_iter()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect()
            })
            .collect();

        let height = matrix.len();
        let width = matrix.get(0).map(|row| row.len()).unwrap_or_default();

        let mut columns = vec![HashMap::with_capacity(height); width];
        let mut rows = vec![HashMap::with_capacity(width); height];

        for y in 0..height {
            for x in 0..width {
                columns[x].insert(matrix[y][x], false);
                rows[y].insert(matrix[y][x], false);
            }
        }

        Self {
            column_bingo: false,
            row_bingo: false,
            columns,
            rows,
        }
    }

    fn mark(&mut self, number: usize) -> Option<usize> {
        self.columns.iter_mut().for_each(|column| {
            if self.column_bingo || self.row_bingo {
                return;
            }

            if let Some(marked) = column.get_mut(&number) {
                *marked = true;
            }

            self.column_bingo = column.values().all(|marked| *marked);
        });

        self.rows.iter_mut().for_each(|row| {
            if self.column_bingo || self.row_bingo {
                return;
            }

            if let Some(marked) = row.get_mut(&number) {
                *marked = true;
            }

            self.row_bingo = row.values().all(|marked| *marked);
        });

        match (self.column_bingo, self.row_bingo) {
            (true, _) => Some(Self::score(&self.columns, number)),
            (_, true) => Some(Self::score(&self.rows, number)),
            _ => None,
        }
    }

    fn scalar_score(scalar: &HashMap<usize, bool>) -> usize {
        scalar
            .keys()
            .filter(|k| !scalar.get(&k).map(|v| *v).unwrap_or_default())
            .sum::<usize>()
    }

    fn score(matrix: &Vec<HashMap<usize, bool>>, number: usize) -> usize {
        matrix.iter().map(Self::scalar_score).sum::<usize>() * number
    }
}

fn parse_input(mut input: Vec<String>) -> (Vec<usize>, Vec<BingoCard>) {
    let calls = input
        .remove(0)
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();

    let lines: Vec<String> = input.into_iter().filter(|l| !l.is_empty()).collect();

    let cards = lines
        .chunks(5)
        .map(|lines| BingoCard::new(lines.to_vec()))
        .collect();

    (calls, cards)
}

fn calculate_first_winning_score(input: Vec<String>) -> usize {
    let (calls, mut cards) = parse_input(input);

    calls
        .into_iter()
        .find_map(|number| cards.iter_mut().find_map(|card| card.mark(number)))
        .unwrap_or_default()
}

fn calculate_last_winning_score(input: Vec<String>) -> usize {
    let (calls, mut cards) = parse_input(input);
    let mut score = None;

    for number in calls {
        cards = cards
            .into_iter()
            .filter_map(|mut card| {
                if let Some(card_score) = card.mark(number) {
                    score = Some(card_score);
                    None
                } else {
                    Some(card)
                }
            })
            .collect();

        if cards.is_empty() {
            break;
        }
    }

    score.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_four.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_calculate_first_winning_score_with_example_input() {
        let input = use_example_input();
        let expected = 4512;
        let actual = super::calculate_first_winning_score(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_first_winning_score_with_real_input() {
        let input = use_real_input();
        let expected = 41668;
        let actual = super::calculate_first_winning_score(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_last_winning_score_with_example_input() {
        let input = use_example_input();
        let expected = 1924;
        let actual = super::calculate_last_winning_score(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_last_with_real_input() {
        let input = use_real_input();
        let expected = 10478;
        let actual = super::calculate_last_winning_score(input);

        assert_eq!(expected, actual);
    }
}
