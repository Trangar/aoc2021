use std::time::Instant;

fn main() {
    let test = include_str!("../../../input/day_11_test.txt")
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect::<Vec<Vec<u8>>>();
    let input = include_str!("../../../input/day_11_input.txt")
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect::<Vec<Vec<u8>>>();

    assert_eq!(1656, dbg!(part_1(test.clone())));
    dbg!(part_1(input.clone()));
    assert_eq!(195, dbg!(part_2(test)));
    dbg!(part_2(input));
}

fn part_1(mut lines: Vec<Vec<u8>>) -> usize {
    let start = Instant::now();
    let mut total = 0;
    for _ in 0..100 {
        total += increase_and_flash(&mut lines);
    }
    println!("Part 1: {:?}", start.elapsed());
    total
}
fn part_2(mut lines: Vec<Vec<u8>>) -> usize {
    let start = Instant::now();
    let expected = lines.iter().map(|row| row.len()).sum::<usize>();
    let mut it = 0;
    loop {
        it += 1;
        let flashes = increase_and_flash(&mut lines);
        if flashes == expected {
            println!("Part 2: {:?}", start.elapsed());
            return it;
        }
    }
}

fn increase_and_flash(grid: &mut [Vec<u8>]) -> usize {
    let mut flashes = Vec::new();
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            grid[x][y] += 1;
            if grid[x][y] > 9 {
                // flash
                flashes.push((x, y));
            }
        }
    }
    let mut flash_idx = 0;
    while flash_idx < flashes.len() {
        let (x, y) = flashes[flash_idx];
        flash_idx += 1;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let (x, y) = (x as i32 + dx, y as i32 + dy);
                if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                grid[x][y] += 1;
                if grid[x][y] == 10 {
                    // it was 9 but now it flashed
                    flashes.push((x, y));
                }
            }
        }
    }
    let flash_count = flashes.len();
    for (x, y) in flashes {
        debug_assert!(grid[x][y] > 9);
        grid[x][y] = 0;
    }
    flash_count
}
