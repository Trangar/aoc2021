fn main() {
    let test: Vec<u32> = include_str!("../../../input/day_7_test.txt")
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let input: Vec<u32> = include_str!("../../../input/day_7_input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    assert_eq!(37, dbg!(part_1(&test)));
    assert_eq!(349769, dbg!(part_1(&input)));
    assert_eq!(168, dbg!(part_2(&test)));
    let start = std::time::Instant::now();
    assert_eq!(99540554, dbg!(part_2(&input)));
    dbg!(start.elapsed());
}

fn part_1(numbers: &[u32]) -> u32 {
    let max = numbers.iter().copied().max().unwrap();
    let mut lowest_fuel = (u32::MAX, 0);
    for i in 0..=max {
        let fuel_count: u32 = numbers
            .iter()
            .map(|num| (*num as i32 - i as i32).unsigned_abs())
            .sum();
        if fuel_count < lowest_fuel.0 {
            lowest_fuel = (fuel_count, i);
        }
    }
    lowest_fuel.0
}

fn part_2(numbers: &[u32]) -> u32 {
    let max = numbers.iter().copied().max().unwrap();
    let mut lowest_fuel = (u32::MAX, 0);
    for i in 0..=max {
        let fuel_count: u32 = numbers
            .iter()
            .map(|num| {
                let distance = (*num as i32 - i as i32).unsigned_abs();

                // (0..=distance).sum::<u32>()

                // Dr Gauss in the house!
                distance * (distance + 1) / 2
            })
            .sum();
        if fuel_count < lowest_fuel.0 {
            lowest_fuel = (fuel_count, i);
        }
    }
    lowest_fuel.0
}
