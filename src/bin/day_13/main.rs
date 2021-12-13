use std::time::Instant;

use bitvec::prelude::BitVec;

fn main() {
    let test = Paper::new(include_str!("../../../input/day_13_test.txt"));
    let input = Paper::new(include_str!("../../../input/day_13_input.txt"));
    assert_eq!(17, dbg!(part_1(&test)));
    dbg!(part_1(&input));

    part_2(&test);
    part_2(&input);
}

fn part_1(paper: &Paper) -> usize {
    let start = Instant::now();
    let mut rows = paper.rows.clone();
    fold(&mut rows, paper.folds[0].0, paper.folds[0].1);

    let result = rows.iter().map(|r| r.count_ones()).sum();
    dbg!(start.elapsed());
    result
}
fn part_2(paper: &Paper) {
    let start = Instant::now();
    let mut rows = paper.rows.clone();
    for (axis, idx) in paper.folds.iter().copied() {
        fold(&mut rows, axis, idx);
    }
    print(&rows);
    dbg!(start.elapsed());
}

fn fold(rows: &mut Vec<BitVec>, axis: char, idx: usize) {
    match axis {
        'x' => {
            for row in rows {
                let rhs = row.drain(idx..).skip(1).rev().collect::<BitVec>();
                assert_eq!(rhs.len(), row.len());

                *row |= rhs;
            }
        }
        'y' => {
            let bottom = rows.drain(idx..).skip(1).rev().collect::<Vec<_>>();
            let offset = rows.len() - bottom.len();

            for (idx, row) in bottom.into_iter().enumerate() {
                rows[offset + idx] |= row;
            }
        }
        x => panic!("Unknown fold: {:?}", x),
    }
}

fn print(rows: &[BitVec]) {
    for row in rows {
        for b in row.iter() {
            if *b {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[derive(Debug)]
struct Paper {
    rows: Vec<BitVec>,
    folds: Vec<(char, usize)>,
}

impl Paper {
    pub fn new(input: &str) -> Self {
        let mut rows = Vec::<BitVec>::new();

        let mut lines = input.lines();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                // empty line between positions and folds
                break;
            }
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            if rows.len() <= y {
                let width = if rows.is_empty() { 0 } else { rows[0].len() };
                rows.resize_with(y + 1, || {
                    let mut vec = BitVec::new();
                    vec.resize(width, false);
                    vec
                });
            }
            if rows[0].len() <= x {
                for row in &mut rows {
                    row.resize(x + 1, false);
                }
            }
            rows[y].set(x, true);
        }
        let mut folds = Vec::new();
        for line in lines {
            if let Some(line) = line.strip_prefix("fold along ") {
                let (axis, value) = line.split_once('=').unwrap();
                folds.push((axis.chars().next().unwrap(), value.parse().unwrap()));
            } else {
                dbg!(line);
                break;
            }
        }

        Self { rows, folds }
    }
}
