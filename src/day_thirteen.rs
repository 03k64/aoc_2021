use crate::position::Position;
use std::str::FromStr;

type Dot = Position;

type DotMap = Vec<Vec<Option<Dot>>>;

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = s.split_once('=').unwrap_or_default();
        let value = value.parse().unwrap_or_default();

        match axis {
            "x" => Ok(Self::X(value)),
            "y" => Ok(Self::Y(value)),
            _ => Err(()),
        }
    }
}

fn fold_left(map: &mut DotMap, height: usize, width: usize) {
    let start = width + 1;
    let end = start + width;

    (0..height).for_each(|y| {
        (start..end).for_each(|x| {
            let target_col = get_folded_coordinate(x, width);
            map[y][target_col] = map[y][target_col].or(map[y][x]);
        });
        map[y].truncate(width);
    });
}

fn fold_up(map: &mut DotMap, width: usize, height: usize) {
    let start = height + 1;
    let end = start + height;

    // 'fold' rows up
    (start..end).for_each(|y| {
        (0..width).for_each(|x| {
            let target_row = get_folded_coordinate(y, height);
            map[target_row][x] = map[target_row][x].or(map[y][x]);
        });
    });

    map.truncate(height);
}

fn get_current_dimensions(map: &DotMap) -> (usize, usize) {
    (
        map.len(),
        map.get(0).map(|row| row.len()).unwrap_or_default(),
    )
}

fn get_folded_coordinate(coordinate: usize, limit: usize) -> usize {
    if coordinate % limit == 0 {
        0
    } else {
        limit - coordinate % limit
    }
}

fn get_initial_dimensions(dots: &Vec<Dot>) -> (usize, usize) {
    let (max_y, max_x) = dots.iter().fold((0, 0), |(height, width), dot| {
        (dot.y.max(height), dot.x.max(width))
    });

    (max_y + 1, max_x + 1)
}

fn parse_input(input: Vec<String>) -> (Vec<Dot>, Vec<Fold>) {
    input
        .into_iter()
        .fold((vec![], vec![]), |(mut dots, mut folds), line| {
            if line.starts_with("fold") {
                let (_, fold) = line.rsplit_once(' ').unwrap_or_default();
                let fold = fold.parse().unwrap();
                folds.push(fold);
            } else {
                let (x, y) = line.split_once(',').unwrap_or_default();
                let x = x.parse().unwrap_or_default();
                let y = y.parse().unwrap_or_default();
                dots.push(Dot { x, y });
            }

            (dots, folds)
        })
}

fn fold_paper(input: Vec<String>, num_folds: Option<usize>) -> DotMap {
    let (dots, folds) = parse_input(input);
    let (height, width) = get_initial_dimensions(&dots);

    let mut map = vec![vec![None; width]; height];
    dots.into_iter().for_each(|dot| {
        map[dot.y][dot.x] = Some(dot);
    });

    let max_fold = num_folds.unwrap_or(folds.len());
    (0..max_fold).for_each(|ix| {
        let (curr_h, curr_w) = get_current_dimensions(&map);

        if let Some(Fold::X(x)) = folds.get(ix) {
            fold_left(&mut map, curr_h, *x);
        }

        if let Some(Fold::Y(y)) = folds.get(ix) {
            fold_up(&mut map, curr_w, *y);
        }
    });

    map
}

fn count_visible_dots(input: Vec<String>, num_folds: usize) -> usize {
    let map = fold_paper(input, Some(num_folds));

    map.into_iter().fold(0, |sum, row| {
        sum + row.into_iter().filter(|d| d.is_some()).count()
    })
}

#[cfg(test)]
mod tests {
    fn use_example_input() -> Vec<String> {
        String::from(
            r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#,
        )
        .lines()
        .map(String::from)
        .filter(|l| !l.is_empty())
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_thirteen.txt")
            .lines()
            .map(String::from)
            .filter(|l| !l.is_empty())
            .collect()
    }

    #[test]
    fn test_count_visible_dots_after_one_fold_with_example_input() {
        let input = use_example_input();
        let expected = 17;
        let actual = super::count_visible_dots(input, 1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_visible_dots_after_one_fold_with_real_input() {
        let input = use_real_input();
        let expected = 666;
        let actual = super::count_visible_dots(input, 1);

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_count_visible_dots_after_all_folds_with_example_input() {
    //     let input = use_example_input();
    //     let expected = 0;
    //     let actual = super::count_visible_dots(input, None);

    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_count_visible_dots_after_all_folds_with_real_input() {
    //     let input = use_real_input();
    //     let expected = 0;
    //     let actual = super::count_visible_dots(input, None);

    //     assert_eq!(expected, actual);
    // }
}
