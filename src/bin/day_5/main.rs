fn main() {
    let test = Grid::parse(include_str!("../../../input/day_5_test.txt"));
    let input = Grid::parse(include_str!("../../../input/day_5_input.txt"));

    assert_eq!(5, dbg!(part_1(&test)));
    dbg!(part_1(&input));
    assert_eq!(12, dbg!(part_2(&test)));
    dbg!(part_2(&input));
}

fn part_1(grid: &Grid) -> usize {
    let mut mapped = vec![vec![0u16; grid.max.1 + 1]; grid.max.0 + 1];
    for line in &grid.lines {
        line.hor_vert_points_callback(|x, y| {
            mapped[x][y] += 1;
        });
    }
    // for y in 0..grid.max.1 + 1 as usize {
    // 	for x in 0..grid.max.0 + 1 as usize {
    // 		if mapped[x][y] == 0 {
    // 			print!(".");
    // 		} else {
    // 			print!("{}", mapped[x][y]);
    // 		}
    // 	}
    // 	println!();
    // }
    mapped.iter().flatten().filter(|n| **n >= 2).count()
}
fn part_2(grid: &Grid) -> usize {
    let mut mapped = vec![vec![0u16; grid.max.1 + 1]; grid.max.0 + 1];
    for line in &grid.lines {
        line.hor_vert_diag_points_callback(|x, y| {
            mapped[x][y] += 1;
        });
    }
    // for y in 0..grid.max.1 + 1 as usize {
    // 	for x in 0..grid.max.0 + 1 as usize {
    // 		if mapped[x][y] == 0 {
    // 			print!(".");
    // 		} else {
    // 			print!("{}", mapped[x][y]);
    // 		}
    // 	}
    // 	println!();
    // }
    mapped.iter().flatten().filter(|n| **n >= 2).count()
}

struct Grid {
    lines: Vec<Line>,
    max: (usize, usize),
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut lines = Vec::new();
        let mut max = (0, 0);
        for line in input.lines() {
            let mut parts = line
                .split(&[',', '-', '>', ' '][..])
                .filter(|s| !s.trim().is_empty());
            let line = Line {
                start: (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                ),
                end: (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                ),
            };
            max = (
                max.0.max(line.start.0 as usize).max(line.end.0 as usize),
                max.1.max(line.start.1 as usize).max(line.end.1 as usize),
            );
            lines.push(line);
        }

        Self { lines, max }
    }
}

struct Line {
    start: (u16, u16),
    end: (u16, u16),
}

impl Line {
    fn hor_vert_points_callback(&self, mut cb: impl FnMut(usize, usize)) {
        let start = self.start;
        let end = self.end;
        if start.0 == end.0 {
            let min = start.1.min(end.1);
            let max = start.1.max(end.1);
            for y in min..=max {
                cb(start.0 as usize, y as usize);
            }
        } else if start.1 == end.1 {
            let min = start.0.min(end.0);
            let max = start.0.max(end.0);
            for x in min..=max {
                cb(x as usize, start.1 as usize);
            }
        }
    }
    fn hor_vert_diag_points_callback(&self, mut cb: impl FnMut(usize, usize)) {
        let start = self.start;
        let end = self.end;
        if start.0 == end.0 {
            let min = start.1.min(end.1);
            let max = start.1.max(end.1);
            for y in min..=max {
                cb(start.0 as usize, y as usize);
            }
        } else if start.1 == end.1 {
            let min = start.0.min(end.0);
            let max = start.0.max(end.0);
            for x in min..=max {
                cb(x as usize, start.1 as usize);
            }
        } else {
            let (start, end) = if start.0 < end.0 {
                (start, end)
            } else {
                (end, start)
            };
            let delta: i16 = if start.1 < end.1 { 1 } else { -1 };
            for step in 0..=end.0 - start.0 {
                let x = start.0 + step;
                let y = (start.1 as i16 + delta * step as i16) as u16;
                cb(x as usize, y as usize);
            }
        }
    }
}
