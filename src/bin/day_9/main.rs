fn main() {
    let test = Heightmap::new(include_str!("../../../input/day_9_test.txt"));
    let input = Heightmap::new(include_str!("../../../input/day_9_input.txt"));

    assert_eq!(15, dbg!(part_1(&test)));
    assert_eq!(535, dbg!(part_1(&input)));
    assert_eq!(1134, dbg!(part_2(&test)));
    assert_eq!(1122700, dbg!(part_2(&input)));
}

fn part_1(map: &Heightmap) -> usize {
    map.lowest_points()
        .into_iter()
        .map(|(x, y)| map.get(x, y).unwrap() as usize + 1)
        .sum()
}

fn part_2(map: &Heightmap) -> usize {
    let mut basin_sizes = Vec::new();
    for point in map.lowest_points() {
        let mut basin = Vec::new();
        let mut wall = Vec::new();
        let mut to_check = vec![point];

        while let Some(point) = to_check.pop() {
            for point in [
                (point.0 - 1, point.1),
                (point.0 + 1, point.1),
                (point.0, point.1 - 1),
                (point.0, point.1 + 1),
            ] {
                if basin.contains(&point) || to_check.contains(&point) || wall.contains(&point) {
                    continue;
                }
                if let Some(val) = map.get(point.0, point.1) {
                    if val == 9 {
                        wall.push(point);
                    } else {
                        to_check.push(point);
                        basin.push(point);
                    }
                }
            }
        }
        basin_sizes.push(basin.len());
        // println!("Basin at {:?} (size: {}) contains the following points:", point, basin.len());
        // for p in basin {
        // 	println!(" - {:?}: ({})", p, map.get(p.0, p.1).unwrap());
        // }
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

struct Heightmap {
    points: Vec<Vec<u8>>,
}

impl Heightmap {
    fn new(input: &str) -> Self {
        Self {
            points: input
                .lines()
                .map(|l| l.bytes().map(|b| b - b'0').collect())
                .collect(),
        }
    }

    // fn print(&self) {
    // 	for row in &self.points {
    // 		for cell in row {
    // 			print!("{}", cell);
    // 		}
    // 		println!();
    // 	}
    // }

    fn get(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            None
        } else {
            Some(self.points[y as usize][x as usize])
        }
    }

    fn width(&self) -> i32 {
        self.points[0].len() as i32
    }
    fn height(&self) -> i32 {
        self.points.len() as i32
    }

    fn lowest_points(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        for x in 0..self.width() {
            for y in 0..self.height() {
                let val = self.get(x, y).unwrap();
                if let Some(left) = self.get(x - 1, y) {
                    if left <= val {
                        continue;
                    }
                }
                if let Some(right) = self.get(x + 1, y) {
                    if right <= val {
                        continue;
                    }
                }
                if let Some(up) = self.get(x, y - 1) {
                    if up <= val {
                        continue;
                    }
                }
                if let Some(down) = self.get(x, y + 1) {
                    if down <= val {
                        continue;
                    }
                }
                points.push((x, y));
            }
        }
        points
    }
}
