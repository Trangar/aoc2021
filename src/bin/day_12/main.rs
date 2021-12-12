use std::time::Instant;

fn main() {
    let test_1 = CaveSystem::new(include_str!("../../../input/day_12_test_1.txt"));
    let test_2 = CaveSystem::new(include_str!("../../../input/day_12_test_2.txt"));
    let test_3 = CaveSystem::new(include_str!("../../../input/day_12_test_3.txt"));
    let input = CaveSystem::new(include_str!("../../../input/day_12_input.txt"));

    assert_eq!(10, dbg!(part_1(&test_1)));
    assert_eq!(19, dbg!(part_1(&test_2)));
    assert_eq!(226, dbg!(part_1(&test_3)));
    dbg!(part_1(&input));

    assert_eq!(36, dbg!(part_2(&test_1)));
    assert_eq!(103, dbg!(part_2(&test_2)));
    assert_eq!(3509, dbg!(part_2(&test_3)));
    dbg!(part_2(&input));
}

fn part_1(system: &CaveSystem) -> usize {
    let start = Instant::now();
    let mut paths_done = Vec::new();
    let mut paths_todo = vec![vec![system.start]];

    while let Some(path) = paths_todo.pop() {
        let last_point = *path.last().unwrap();
        for adjacent in system.points[last_point].1.iter().copied() {
            let mut adjacent_path = path.clone();
            adjacent_path.push(adjacent);
            if paths_done.contains(&adjacent_path) || paths_todo.contains(&adjacent_path) {
            } else if system.path_is_valid(&adjacent_path) {
                if system.end == adjacent {
                    paths_done.push(adjacent_path);
                } else {
                    paths_todo.push(adjacent_path);
                }
            }
        }
    }
    println!("Part 1 done in {:?}", start.elapsed());
    paths_done.len()
}

fn part_2(system: &CaveSystem) -> usize {
    let start = Instant::now();
    let mut paths_done = Vec::new();
    let mut paths_todo = vec![vec![system.start]];

    while let Some(path) = paths_todo.pop() {
        let last_point = *path.last().unwrap();
        for adjacent in system.points[last_point].1.iter().copied() {
            let mut adjacent_path = path.clone();
            adjacent_path.push(adjacent);
            if paths_done.contains(&adjacent_path) || paths_todo.contains(&adjacent_path) {
            } else if system.path_is_valid_with_revisit(&adjacent_path) {
                if system.end == adjacent {
                    paths_done.push(adjacent_path);
                } else {
                    paths_todo.push(adjacent_path);
                }
            }
        }
    }
    println!("Part 2 done in {:?}", start.elapsed());
    paths_done.len()
}

#[derive(Clone, Debug)]
struct CaveSystem {
    points: Vec<(&'static str, Vec<usize>, CaveType)>,
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CaveType {
    StartEnd,
    Small,
    Big,
}

impl CaveType {
    fn from_name(name: &str) -> Self {
        if name == "start" || name == "end" {
            Self::StartEnd
        } else if name.chars().all(|c| c.is_lowercase()) {
            Self::Small
        } else {
            Self::Big
        }
    }
}

impl CaveSystem {
    fn new(input: &'static str) -> Self {
        let mut points = Vec::new();
        for line in input.lines() {
            let (name, other) = line.split_once('-').unwrap();
            let name_idx = get_idx_or_insert(&mut points, name);
            let other_idx = get_idx_or_insert(&mut points, other);

            points[name_idx].1.push(other_idx);
            points[other_idx].1.push(name_idx);
        }
        let start = points.iter().position(|(n, _, _)| *n == "start").unwrap();
        let end = points.iter().position(|(n, _, _)| *n == "end").unwrap();
        Self { points, start, end }
    }

    fn path_is_valid(&self, idx: &[usize]) -> bool {
        let mut path_count = vec![0usize; self.points.len()];
        for idx in idx.iter().copied() {
            path_count[idx] += 1;
            if path_count[idx] >= 2 && self.points[idx].2 != CaveType::Big {
                return false;
            }
        }
        true
    }

    fn path_is_valid_with_revisit(&self, idx: &[usize]) -> bool {
        let mut path_count = vec![0usize; self.points.len()];
        let mut first_duplicate = true;
        for idx in idx.iter().copied() {
            path_count[idx] += 1;
            if path_count[idx] >= 2 {
                match self.points[idx].2 {
                    CaveType::StartEnd => return false,
                    CaveType::Small if first_duplicate => {
                        first_duplicate = false;
                    }
                    CaveType::Small => return false,
                    _ => {}
                }
            }
        }

        true
    }
}

fn get_idx_or_insert(
    points: &mut Vec<(&'static str, Vec<usize>, CaveType)>,
    name: &'static str,
) -> usize {
    if let Some(idx) = points.iter().position(|(n, _, _)| *n == name) {
        idx
    } else {
        let len = points.len();
        points.push((name, Vec::new(), CaveType::from_name(name)));
        len
    }
}
