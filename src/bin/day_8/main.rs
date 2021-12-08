fn main() {
    let test = include_str!("../../../input/day_8_test.txt")
        .lines()
        .map(TestCase::parse)
        .collect::<Vec<_>>();
    let input = include_str!("../../../input/day_8_input.txt")
        .lines()
        .map(TestCase::parse)
        .collect::<Vec<_>>();

    assert_eq!(26, dbg!(part_1(&test)));
    dbg!(part_1(&input));
    assert_eq!(
        5353,
        dbg!(part_2(&[TestCase::parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        )]))
    );
    assert_eq!(61229, dbg!(part_2(&test)));
    dbg!(part_2(&input));
}

fn part_1(cases: &[TestCase]) -> usize {
    cases
        .iter()
        .map(|case| {
            case.values
                .iter()
                .filter(|v| [2, 3, 4, 7].contains(&v.len()))
                .count()
        })
        .sum()
}

fn part_2(cases: &[TestCase]) -> u32 {
    let mut result = 0;
    for case in cases {
        let mut segment = Segment::default();
        let all_values = case
            .test_cases
            .iter()
            .chain(case.values.iter())
            .collect::<Vec<_>>();
        while !segment.solved() {
            let start = segment.clone();
            for value in &all_values {
                if value.len() == 2 {
                    segment.solve_1(value);
                } else if value.len() == 3 {
                    segment.solve_7(value);
                } else if value.len() == 4 {
                    segment.solve_4(value);
                } else if value.len() == 5 {
                    segment.solve_2_3_or_5(value);
                } else if value.len() == 6 {
                    segment.solve_0_6_or_9(value);
                } else if value.len() == 7 {
                    // eight doesn't do anything
                } else {
                    unimplemented!("Len {}, digits: {}", value.len(), value);
                }
            }
            segment.clear_solved();

            if start == segment {
                segment.print();
                panic!("Solution stalled!");
            }
        }

        let number = segment.lookup_value(case.values[0]) * 1000
            + segment.lookup_value(case.values[1]) * 100
            + segment.lookup_value(case.values[2]) * 10
            + segment.lookup_value(case.values[3]);
        result += number;
    }

    result
}

#[derive(PartialEq, Clone)]
struct Segment {
    segments: [Vec<char>; 7],
}

impl Default for Segment {
    fn default() -> Self {
        let mut segments: [Vec<char>; 7] = Default::default();
        for segment in &mut segments {
            *segment = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        }
        Self { segments }
    }
}

impl Segment {
    fn solved(&self) -> bool {
        self.segments.iter().all(|s| s.len() == 1)
    }

    fn solve_1(&mut self, numbers: &str) {
        self.digit(
            &[SegmentIndex::TopRight, SegmentIndex::BottomRight],
            numbers,
        );
    }
    fn solve_4(&mut self, numbers: &str) {
        self.digit(
            &[
                SegmentIndex::TopLeft,
                SegmentIndex::TopRight,
                SegmentIndex::Middle,
                SegmentIndex::BottomRight,
            ],
            numbers,
        );
    }
    fn solve_7(&mut self, numbers: &str) {
        self.digit(
            &[
                SegmentIndex::Top,
                SegmentIndex::TopRight,
                SegmentIndex::BottomRight,
            ],
            numbers,
        );
    }

    fn solve_0_6_or_9(&mut self, numbers: &str) {
        // the charcters in `numbers` *must* be in the following places:
        let segments = &[
            SegmentIndex::Top,
            SegmentIndex::TopLeft,
            SegmentIndex::BottomRight,
            SegmentIndex::Bottom,
        ];
        for segment in segments.iter().copied() {
            self.segments[segment as usize].retain(|c| numbers.chars().any(|n| *c == n));
        }
    }

    fn solve_2_3_or_5(&mut self, numbers: &str) {
        // the charcters in `numbers` *must* be in the following places:
        let segments = &[
            SegmentIndex::Top,
            SegmentIndex::Middle,
            SegmentIndex::Bottom,
        ];
        for segment in segments.iter().copied() {
            self.segments[segment as usize].retain(|c| numbers.chars().any(|n| *c == n));
        }
    }

    fn clear_solved(&mut self) {
        let segments = self.segments.clone();
        for (idx, segment) in segments.into_iter().enumerate() {
            if segment.len() == 1 {
                // remove this value from all other segments
                for other_idx in SegmentIndex::all() {
                    let other_idx = *other_idx as usize;
                    if idx == other_idx {
                        continue;
                    }
                    self.segments[other_idx].retain(|c| *c != segment[0]);
                }
            }
        }
    }

    fn digit(&mut self, matches: &[SegmentIndex], numbers: &str) {
        // Remove all chars from `matches` that aren't in `numbers`
        for m in matches.iter().copied() {
            self.segments[m as usize].retain(|c| numbers.chars().any(|n| n == *c));
            if self.segments[m as usize].is_empty() {
                panic!("Segment {:?} is empty", m);
            }
        }

        // From all other segments, remove all chars in `numbers`
        for (idx, segment) in self.segments.iter_mut().enumerate() {
            if matches.iter().copied().any(|m| m as usize == idx) {
                continue;
            }
            segment.retain(|c| numbers.chars().all(|n| n != *c));
            if segment.is_empty() {
                panic!("Segment {:?} is empty", idx);
            }
        }
    }

    fn print(&self) {
        let top = self.segments[SegmentIndex::Top as usize][0];
        let top_left = self.segments[SegmentIndex::TopLeft as usize][0];
        let top_right = self.segments[SegmentIndex::TopRight as usize][0];
        let middle = self.segments[SegmentIndex::Middle as usize][0];
        let bottom_left = self.segments[SegmentIndex::BottomLeft as usize][0];
        let bottom_right = self.segments[SegmentIndex::BottomRight as usize][0];
        let bottom = self.segments[SegmentIndex::Bottom as usize][0];
        println!(" {0}{0}{0}{0}", top);
        println!("{0}    {1}", top_left, top_right);
        println!("{0}    {1}", top_left, top_right);
        println!(" {0}{0}{0}{0}", middle);
        println!("{0}    {1}", bottom_left, bottom_right);
        println!("{0}    {1}", bottom_left, bottom_right);
        println!(" {0}{0}{0}{0}", bottom);
    }

    fn lookup_value(&self, str: &str) -> u32 {
        use SegmentIndex::*;
        let mut indices = str
            .chars()
            .map(|c| SegmentIndex::from_num(self.segments.iter().position(|l| l[0] == c).unwrap()))
            .collect::<Vec<_>>();
        indices.sort();
        match indices.as_slice() {
            &[Top, TopLeft, TopRight, BottomLeft, BottomRight, Bottom] => 0,
            &[TopRight, BottomRight] => 1,
            &[Top, TopRight, Middle, BottomLeft, Bottom] => 2,
            &[Top, TopRight, Middle, BottomRight, Bottom] => 3,
            &[TopLeft, TopRight, Middle, BottomRight] => 4,
            &[Top, TopLeft, Middle, BottomRight, Bottom] => 5,
            &[Top, TopLeft, Middle, BottomLeft, BottomRight, Bottom] => 6,
            &[Top, TopRight, BottomRight] => 7,
            slice if slice.len() == 7 => 8,
            &[Top, TopLeft, TopRight, Middle, BottomRight, Bottom] => 9,
            x => panic!("Unknown pattern: {:?}", x),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
enum SegmentIndex {
    Top = 0,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

impl SegmentIndex {
    fn from_num(num: usize) -> Self {
        match num {
            0 => Self::Top,
            1 => Self::TopLeft,
            2 => Self::TopRight,
            3 => Self::Middle,
            4 => Self::BottomLeft,
            5 => Self::BottomRight,
            6 => Self::Bottom,
            _ => unreachable!(),
        }
    }
    fn all() -> &'static [SegmentIndex] {
        &[
            Self::Top,
            Self::TopLeft,
            Self::TopRight,
            Self::Middle,
            Self::BottomLeft,
            Self::BottomRight,
            Self::Bottom,
        ]
    }
}

pub struct TestCase {
    test_cases: Vec<&'static str>,
    values: Vec<&'static str>,
}

impl TestCase {
    fn parse(s: &'static str) -> Self {
        let (left, right) = s.split_once('|').unwrap();
        Self {
            test_cases: left
                .split_whitespace()
                .filter_map(|s| {
                    let s = s.trim();
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                })
                .collect(),
            values: right
                .split_whitespace()
                .filter_map(|s| {
                    let s = s.trim();
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                })
                .collect(),
        }
    }
}
