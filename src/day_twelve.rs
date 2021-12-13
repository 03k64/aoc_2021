use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy)]
enum CaveSize {
    Big,
    Small,
}

#[derive(Clone)]
struct Cave {
    id: String,
    links: Vec<String>,
    size: CaveSize,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = if s == s.to_uppercase() {
            CaveSize::Big
        } else {
            CaveSize::Small
        };

        Ok(Self {
            id: s.to_owned(),
            links: vec![],
            size,
        })
    }
}

impl Cave {
    fn valid_next_caves<P>(&self, path: &Vec<String>) -> Option<Vec<&String>>
    where
        P: Path,
    {
        let valid_links: Vec<&String> = self
            .links
            .iter()
            .filter(|l| P::is_valid_next_cave(l, path))
            .collect();

        if valid_links.is_empty() {
            None
        } else {
            Some(valid_links)
        }
    }
}

trait Path {
    fn is_valid_next_cave(id: &String, path: &Vec<String>) -> bool;
}

struct SimplePath;

impl Path for SimplePath {
    fn is_valid_next_cave(id: &String, path: &Vec<String>) -> bool {
        *id == id.to_uppercase() || !path.contains(id)
    }
}

struct ComplexPath;

impl Path for ComplexPath {
    fn is_valid_next_cave(id: &String, path: &Vec<String>) -> bool {
        if SimplePath::is_valid_next_cave(id, path) {
            return true;
        }

        let mut small_visited: Vec<&String> =
            path.iter().filter(|v| **v == v.to_lowercase()).collect();

        small_visited.sort();

        let small_visited_count = small_visited.len();

        small_visited.dedup();
        let uniq_small_visited_count = small_visited.len();

        small_visited_count == uniq_small_visited_count
    }
}

fn parse_input(input: Vec<String>) -> HashMap<String, Cave> {
    input.into_iter().fold(HashMap::new(), |mut caves, line| {
        let (a, b) = line.split_once('-').unwrap_or_default();

        let cave = caves
            .entry(a.to_string())
            .or_insert(a.parse::<Cave>().unwrap());

        if a != "end" && b != "start" {
            cave.links.push(b.to_string());
        }

        let cave = caves
            .entry(b.to_string())
            .or_insert(b.parse::<Cave>().unwrap());

        if b != "end" && a != "start" {
            cave.links.push(a.to_string());
        }

        caves
    })
}

fn find_path<P>(
    current: &Cave,
    mut path: Vec<String>,
    paths: &mut Vec<Vec<String>>,
    caves: &HashMap<String, Cave>,
) where
    P: Path,
{
    path.push(current.id.clone());

    if current.id.as_str() == "end" {
        paths.push(path);
        return;
    }

    if let Some(links) = current.valid_next_caves::<P>(&path) {
        for link in links.iter() {
            let next = caves.get(*link).unwrap();
            find_path::<P>(next, path.clone(), paths, caves);
        }
    }
}

fn enumerate_paths<P>(input: Vec<String>) -> Vec<String>
where
    P: Path,
{
    let caves = parse_input(input);

    let start = caves.get("start").unwrap();
    let path = vec![];
    let mut paths = vec![];

    find_path::<P>(start, path, &mut paths, &caves);
    paths.into_iter().map(|path| path.join(",")).collect()
}

#[cfg(test)]
mod tests {
    use super::{ComplexPath, SimplePath};

    fn use_smallest_example_input() -> Vec<String> {
        String::from(
            r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_small_example_input() -> Vec<String> {
        String::from(
            r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_example_input() -> Vec<String> {
        String::from(
            r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#,
        )
        .lines()
        .map(String::from)
        .collect()
    }

    fn use_real_input() -> Vec<String> {
        include_str!("../input/day_twelve.txt")
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_enumerate_simple_paths_with_smallest_example_input() {
        let input = use_smallest_example_input();
        let expected = 10;
        let actual = super::enumerate_paths::<SimplePath>(input);

        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_enumerate_simple_paths_with_small_example_input() {
        let input = use_small_example_input();
        let expected = 19;
        let actual = super::enumerate_paths::<SimplePath>(input);

        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_enumerate_simple_paths_with_example_input() {
        let input = use_example_input();
        let expected = 226;
        let actual = super::enumerate_paths::<SimplePath>(input);

        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_enumerate_simple_paths_with_real_input() {
        let input = use_real_input();
        let expected = 4186;
        let actual = super::enumerate_paths::<SimplePath>(input);

        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_enumerate_complex_paths_with_smallest_example_input() {
        let input = use_smallest_example_input();
        let expected = 36;
        let actual = super::enumerate_paths::<ComplexPath>(input);

        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_enumerate_complex_paths_with_small_example_input() {
        let input = use_small_example_input();
        let expected = 103;
        let actual = super::enumerate_paths::<ComplexPath>(input);

        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_enumerate_complex_paths_with_example_input() {
        let input = use_example_input();
        let expected = 3509;
        let actual = super::enumerate_paths::<ComplexPath>(input);

        assert_eq!(expected, actual.len());
    }

    #[test]
    fn test_enumerate_complex_paths_with_real_input() {
        let input = use_real_input();
        let expected = 92111;
        let actual = super::enumerate_paths::<ComplexPath>(input);

        assert_eq!(expected, actual.len());
    }
}
