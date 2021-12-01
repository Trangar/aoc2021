use aoc2021::input_numbered_lines;

fn main() {
    assert_eq!(7, dbg!(part_1(&input_numbered_lines!("day_1_test_1.txt"))));
    dbg!(part_1(&input_numbered_lines!("day_1_input.txt")));
    assert_eq!(5, dbg!(part_2(&input_numbered_lines!("day_1_test_1.txt"))));
    dbg!(part_2(&input_numbered_lines!("day_1_input.txt")));
}

fn part_1(lines: &[u32]) -> u32 {
    let (mut previous, remaining) = lines.split_first().unwrap();
    let mut increments = 0;
    for num in remaining {
        if num > previous {
            increments += 1;
        }
        previous = num;
    }
    increments
}

fn part_2(lines: &[u32]) -> u32 {
    let mut windows = lines.windows(3).map(|s| s.iter().sum::<u32>());
    let mut previous = windows.next().unwrap();
    windows.fold(0, |mut acc, current| {
        if current > previous {
            acc += 1;
        }
        previous = current;
        acc
    })
}
