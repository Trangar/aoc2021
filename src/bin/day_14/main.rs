use std::{collections::HashMap, time::Instant};

fn main() {
    let test = parse(include_str!("../../../input/day_14_test.txt"));
    let input = parse(include_str!("../../../input/day_14_input.txt"));

    assert_eq!(1588, dbg!(part_1(test.0.clone(), test.1.clone())));
    assert_eq!(3342, dbg!(part_1(input.0.clone(), input.1.clone())));
    assert_eq!(1588, dbg!(part_1_smort(test.0.clone(), test.1.clone())));
    assert_eq!(3342, dbg!(part_1_smort(input.0.clone(), input.1.clone())));
    assert_eq!(2188189693529, dbg!(part_2(test.0, test.1)));
    dbg!(part_2(input.0, input.1));
}

fn part_1(mut input: Vec<u8>, mapping: HashMap<(u8, u8), u8>) -> usize {
    let start = Instant::now();
    for _ in 0..10 {
        // println!("{:?}", std::str::from_utf8(&input).unwrap());
        let mut new_input = Vec::with_capacity(input.len() * 2);

        for pair in input.windows(2) {
            let left = pair[0];
            let right = pair[1];

            new_input.push(left);
            if let Some(mapped) = mapping.get(&(left, right)) {
                new_input.push(*mapped);
            }
        }
        new_input.push(*input.last().unwrap());
        input = new_input;
    }
    // println!("{:?}", std::str::from_utf8(&input).unwrap());

    input.sort_unstable();
    let mut min = (b' ', usize::MAX);
    let mut max = (b' ', 0usize);

    let mut idx = 0;
    while let Some(char) = input.get(idx).copied() {
        let len = input[idx..]
            .iter()
            .position(|c| *c != char)
            .unwrap_or(input.len() - idx);
        idx += len;

        if len < min.1 {
            min = (char, len);
        }
        if len > max.1 {
            max = (char, len);
        }
    }

    print!("Min: {} ({} times), ", min.0 as char, min.1);
    println!("max: {} ({} times)", max.0 as char, max.1);
    dbg!(start.elapsed());

    max.1 - min.1
}

fn part_1_smort(input: Vec<u8>, mapping: HashMap<(u8, u8), u8>) -> usize {
    let start = Instant::now();
    let mut pairs = HashMap::new();
    for pair in input.windows(2) {
        *pairs.entry((pair[0], pair[1])).or_insert(0usize) += 1;
    }

    for _ in 0..10 {
        let mut new = HashMap::new();
        for (pair, &value) in &pairs {
            if let Some(mapped) = mapping.get(pair).copied() {
                *new.entry((pair.0, mapped)).or_insert(0usize) += value;
                *new.entry((mapped, pair.1)).or_insert(0usize) += value;
            }
        }
        pairs = new;
    }
    let mut flattened = HashMap::new();
    for ((left, _right), amount) in pairs {
        *flattened.entry(left).or_insert(0usize) += amount;
    }

    *flattened.get_mut(input.last().unwrap()).unwrap() += 1;

    let mut flattened = flattened.into_iter().collect::<Vec<_>>();
    flattened.sort_unstable_by_key(|(_, len)| *len);

    let (max_char, max_amount) = flattened.last().copied().unwrap();
    let (min_char, min_amount) = flattened.first().copied().unwrap();
    println!(
        "Min: {} ({} times), Max: {} ({} times)",
        min_char as char, min_amount, max_char as char, max_amount
    );
    dbg!(start.elapsed());

    max_amount - min_amount
}

fn part_2(input: Vec<u8>, mapping: HashMap<(u8, u8), u8>) -> usize {
    let start = Instant::now();
    let mut pairs = HashMap::new();
    for pair in input.windows(2) {
        *pairs.entry((pair[0], pair[1])).or_insert(0usize) += 1;
    }

    for _ in 0..40 {
        let mut new = HashMap::new();
        for (pair, &value) in &pairs {
            if let Some(mapped) = mapping.get(pair).copied() {
                *new.entry((pair.0, mapped)).or_insert(0usize) += value;
                *new.entry((mapped, pair.1)).or_insert(0usize) += value;
            }
        }
        pairs = new;
    }
    let mut flattened = HashMap::new();
    for ((left, _right), amount) in pairs {
        *flattened.entry(left).or_insert(0usize) += amount;
    }

    *flattened.get_mut(input.last().unwrap()).unwrap() += 1;

    let mut flattened = flattened.into_iter().collect::<Vec<_>>();
    flattened.sort_unstable_by_key(|(_, len)| *len);

    let (max_char, max_amount) = flattened.last().copied().unwrap();
    let (min_char, min_amount) = flattened.first().copied().unwrap();
    println!(
        "Min: {} ({} times), Max: {} ({} times)",
        min_char as char, min_amount, max_char as char, max_amount
    );
    dbg!(start.elapsed());

    max_amount - min_amount
}

pub fn parse(input: &str) -> (Vec<u8>, HashMap<(u8, u8), u8>) {
    let mut lines = input.lines();
    let input = lines.next().unwrap().as_bytes().to_vec();
    let mut pairs = HashMap::new();
    lines.next();
    for line in lines {
        let (left, right) = line.split_once(" -> ").unwrap();
        if let (&[l1, l2], &[r]) = (left.as_bytes(), right.as_bytes()) {
            pairs.insert((l1, l2), r);
        } else {
            panic!("Unknown left: {:?}, right: {:?}", left, right);
        }
    }

    (input, pairs)
}
