fn main() {
    let test_lines = include_str!("../../../input/day_3_test.txt")
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>();
    let input_lines = include_str!("../../../input/day_3_input.txt")
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>();
    assert_eq!(198, dbg!(part_1(&test_lines)));
    assert_eq!(3959450, dbg!(part_1(&input_lines)));
    assert_eq!(230, dbg!(part_2(&test_lines)));
    assert_eq!(7440311, dbg!(part_2(&input_lines)));
}

fn part_1(numbers: &[&str]) -> u32 {
    let len = numbers[0].len();
    let mut ones = vec![0; len];
    for n in numbers {
        for (offset, c) in n.bytes().enumerate() {
            if c == b'1' {
                ones[offset] += 1;
            }
        }
    }
    let threshold = numbers.len() / 2;
    let mut epsilon = 0;
    let mut gamma = 0;
    for (b, one_count) in ones.into_iter().enumerate() {
        let mask = 1 << (len - b - 1);
        if one_count > threshold {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }
    gamma * epsilon
}

fn num_at(number: &str, offset: usize) -> usize {
    if number.as_bytes()[offset] == b'1' {
        1
    } else {
        0
    }
}

fn count(all_numbers: &[&str], indices: &[usize], offset: usize, num: usize) -> usize {
    let mut count = 0;
    for idx in indices {
        let number = all_numbers[*idx];
        if num_at(number, offset) == num {
            count += 1;
        }
    }
    count
}
fn count_ones(all_numbers: &[&str], indices: &[usize], offset: usize) -> usize {
    count(all_numbers, indices, offset, 1)
}
fn count_zeroes(all_numbers: &[&str], indices: &[usize], offset: usize) -> usize {
    count(all_numbers, indices, offset, 0)
}
fn remove(all_numbers: &[&str], indices: &mut Vec<usize>, offset: usize, num: usize) {
    indices.retain(|idx| num_at(all_numbers[*idx], offset) != num);
}
fn remove_zeroes(all_numbers: &[&str], indices: &mut Vec<usize>, offset: usize) {
    remove(all_numbers, indices, offset, 0);
}
fn remove_ones(all_numbers: &[&str], indices: &mut Vec<usize>, offset: usize) {
    remove(all_numbers, indices, offset, 1);
}

fn part_2(numbers: &[&str]) -> u32 {
    let len = numbers[0].len();
    let all_indices = numbers
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut valid_oxygen_indices = all_indices.clone();
    let mut valid_co2_indices = all_indices;

    for b in 0..len {
        if valid_oxygen_indices.len() > 1 {
            let ones = count_ones(numbers, &valid_oxygen_indices, b);
            if ones * 2 >= valid_oxygen_indices.len() {
                remove_zeroes(numbers, &mut valid_oxygen_indices, b);
            } else {
                remove_ones(numbers, &mut valid_oxygen_indices, b);
            }
        }
        if valid_co2_indices.len() > 1 {
            let ones = count_zeroes(numbers, &valid_co2_indices, b);
            if ones * 2 > valid_co2_indices.len() {
                remove_zeroes(numbers, &mut valid_co2_indices, b);
            } else {
                remove_ones(numbers, &mut valid_co2_indices, b);
            }
        }
    }

    let oxygen = u32::from_str_radix(numbers[valid_oxygen_indices[0]], 2).unwrap();
    let co2 = u32::from_str_radix(numbers[valid_co2_indices[0]], 2).unwrap();

    oxygen * co2
}
