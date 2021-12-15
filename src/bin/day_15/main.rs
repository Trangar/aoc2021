use std::time::Instant;

fn main() {
    let test = Map::new(include_str!("../../../input/day_15_test.txt"));
    let input = Map::new(include_str!("../../../input/day_15_input.txt"));

    assert_eq!(40, dbg!(part_1(&test)));
    dbg!(part_1(&input));

    let test_expanded = test.expand();
    let expected_test_expanded = Map::new(include_str!("../../../input/day_15_test_expanded.txt"));
    assert_eq!(test_expanded, expected_test_expanded);
    let input_expanded = input.expand();

    assert_eq!(315, dbg!(part_1(&test_expanded)));
    dbg!(part_1(&input_expanded));
}

fn part_1(map: &Map) -> usize {
    let start = Instant::now();
    let (width, height) = map.size();
    let mut risk_map = Map::<Option<usize>>::sized(width, height, |_, _| None);

    risk_map.set(0, 0, Some(0));
    loop {
        let mut change_count = 0;
        for x in 0..width {
            for y in 0..height {
                if x == 0 && y == 0 {
                    continue;
                }
                let risk = map.assume(x, y) as usize;
                let mut lowest_neighbour = usize::MAX;
                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    if let Some(Some(neighbour)) = risk_map.get(x as isize + dx, y as isize + dy) {
                        lowest_neighbour = lowest_neighbour.min(neighbour);
                    }
                }
                let risk = risk + lowest_neighbour;
                if let Some(old_risk) = risk_map.assume(x, y) {
                    if old_risk <= risk {
                        continue;
                    }
                }
                risk_map.set(x, y, Some(risk));
                change_count += 1;
            }
        }
        if change_count == 0 {
            break;
        }
    }

    dbg!(start.elapsed());
    risk_map.assume(width - 1, height - 1).unwrap()
}

#[derive(PartialEq, Debug)]
struct Map<T = u8> {
    tiles: Vec<Vec<T>>,
}

impl Map {
    pub fn new(s: &str) -> Self {
        let mut tiles = Vec::new();
        for line in s.lines() {
            tiles.push(line.bytes().map(|b| b - b'0').collect());
        }

        Self { tiles }
    }

    pub fn expand(&self) -> Self {
        let start = Instant::now();
        let (width, height) = self.size();
        let result = Self::sized(width * 5, height * 5, |x, y| {
            let x_factor = x / width;
            let y_factor = y / height;
            let x = x % width;
            let y = y % height;
            let result = self.assume(x, y) as usize + x_factor + y_factor;
            if result > 9 {
                (result - 9) as u8
            } else {
                result as u8
            }
        });
        println!("Expanding took {:?}", start.elapsed());
        result
    }
}
impl<T> Map<T> {
    pub fn sized<FN>(width: usize, height: usize, seed: FN) -> Self
    where
        FN: Fn(usize, usize) -> T,
    {
        let mut tiles = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(seed(x, y));
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.tiles[y][x] = value;
    }

    pub fn size(&self) -> (usize, usize) {
        (self.tiles[0].len(), self.tiles.len())
    }
}

impl<T> Map<T>
where
    T: Copy,
{
    pub fn get(&self, x: isize, y: isize) -> Option<T> {
        if x < 0 || y < 0 {
            None
        } else {
            self.tiles
                .get(y as usize)
                .and_then(|row| row.get(x as usize))
                .copied()
        }
    }
    pub fn assume(&self, x: usize, y: usize) -> T {
        self.tiles[y][x]
    }
}
