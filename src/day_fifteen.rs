use crate::position::Position;
use std::collections::HashMap;

fn generate_part_two_input(input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let repeat = 5;
    let height = input.len();
    let width = input.get(0).map(|row| row.len()).unwrap_or_default();

    let mut output = vec![vec![0; width * repeat]; height * repeat];

    for y in 0..repeat {
        for x in 0..repeat {
            for row in 0..height {
                for col in 0..width {
                    let mut value = input[row][col] + y + x;
                    if value > 9 {
                        value = value % 9;
                    }

                    output[y * height + row][x * width + col] = value;
                }
            }
        }
    }

    output
}

fn orthogonal_distance(current: Position, goal: Position) -> usize {
    (goal.x - current.x) + (goal.y - current.y)
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    mut current: Position,
) -> Vec<Position> {
    let mut path = vec![current];

    while let Some(previous) = came_from.get(&current) {
        current = *previous;
        path.push(current);
    }

    path
}

fn astar<H>(
    start: Position,
    goal: Position,
    heuristic: H,
    height: usize,
    width: usize,
    distances: &Vec<Vec<usize>>,
) -> Vec<Position>
where
    H: Fn(Position, Position) -> usize,
{
    let mut open_set = vec![start];
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, heuristic(start, goal));

    let mut path = vec![];

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .map(|node| (node, f_score.get(&node).map(|f| *f).unwrap_or(usize::MAX)))
            .min_by_key(|(_node, f)| *f)
            .map(|(node, _f)| *node)
            .unwrap();

        if current == goal {
            path = reconstruct_path(&came_from, current);
        }

        let current_ix = open_set.iter().position(|p| *p == current).unwrap();
        open_set.remove(current_ix);

        for neighbour in current.neighbours_orthogonal(height, width) {
            let tentative_gscore = g_score[&current] + distances[neighbour.y][neighbour.x];

            if tentative_gscore < g_score.get(&neighbour).map(|g| *g).unwrap_or(usize::MAX) {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_gscore);
                f_score.insert(neighbour, tentative_gscore + heuristic(neighbour, goal));

                if open_set.iter().find(|p| **p == neighbour).is_none() {
                    open_set.push(neighbour);
                }
            }
        }
    }

    path
}

fn calculate_minimum_total_risk(input: Vec<Vec<usize>>) -> usize {
    let height = input.len();
    let width = input.get(0).map(|row| row.len()).unwrap_or_default();

    let start = Position::default();
    let goal = Position {
        x: width - 1,
        y: height - 1,
    };

    let path = astar(start, goal, orthogonal_distance, height, width, &input);
    let path_risk: usize = path.into_iter().map(|Position { x, y }| input[y][x]).sum();

    path_risk - input[0][0]
}

#[cfg(test)]
mod tests {
    use crate::day_fifteen::generate_part_two_input;

    fn use_part_one_example_input() -> Vec<String> {
        String::from(
            r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_part_two_example_input() -> Vec<String> {
        String::from(
            r#"11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_fifteen.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    fn parse_input(input: Vec<String>) -> Vec<Vec<usize>> {
        input
            .into_iter()
            .map(|row| {
                row.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|d| d as usize)
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_calculate_minimum_total_risk_with_part_one_example_input() {
        let input = use_part_one_example_input();
        let input = parse_input(input);
        let expected = 40;
        let actual = super::calculate_minimum_total_risk(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_minimum_total_risk_with_real_input() {
        let input = use_real_input();
        let input = parse_input(input);
        let expected = 386;
        let actual = super::calculate_minimum_total_risk(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_generate_part_two_input() {
        let input = use_part_one_example_input();
        let input = parse_input(input);
        let expected = use_part_two_example_input();
        let expected = parse_input(expected);
        let actual = super::generate_part_two_input(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_minimum_total_risk_with_part_two_example_input() {
        let input = use_part_two_example_input();
        let input = parse_input(input);
        let expected = 315;
        let actual = super::calculate_minimum_total_risk(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_minimum_total_risk_with_generated_part_two_real_input() {
        let input = use_real_input();
        let input = parse_input(input);
        let input = generate_part_two_input(input);
        let expected = 2806;
        let actual = super::calculate_minimum_total_risk(input);

        assert_eq!(expected, actual);
    }
}
