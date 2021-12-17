use std::time::Instant;

fn main() {
    let test = TargetArea::new(include_str!("../../../input/day_17_test.txt"));
    let input = TargetArea::new(include_str!("../../../input/day_17_input.txt"));

    assert_eq!(45, dbg!(part_1(&test)));
    dbg!(part_1(&input));
    assert_eq!(112, dbg!(part_2(&test)));
    dbg!(part_2(&input));
}

fn part_1(target: &TargetArea) -> usize {
    let start = Instant::now();
    let mut highest_y = 0;
    for x in 1..=target.bottom_right.0 {
        for y in 1..1000 {
            if let HitResult::Hit { highest, .. } = target.is_hit_by((x, y)) {
                highest_y = highest_y.max(highest as usize);
            }
        }
    }
    println!("Part 1: {:?}", start.elapsed());
    highest_y
}

fn part_2(target: &TargetArea) -> usize {
    let start = Instant::now();
    let mut count = 0;
    for x in 1..=target.bottom_right.0 {
        for y in -1000..1000 {
            if let HitResult::Hit { .. } = target.is_hit_by((x, y)) {
                count += 1;
            }
        }
    }
    println!("Part 1: {:?}", start.elapsed());
    count
}

#[derive(Debug)]
struct TargetArea {
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

impl TargetArea {
    fn new(s: &str) -> Self {
        let s = s.strip_prefix("target area: ").unwrap();
        let (x, y) = s.split_once(' ').unwrap();
        let x = &x[2..x.len() - 1].split_once("..").unwrap();
        let y = &y[2..y.len() - 1].split_once("..").unwrap();

        let mut x = (x.0.parse().unwrap(), x.1.parse().unwrap());
        let mut y = (y.0.parse().unwrap(), y.1.parse().unwrap());

        if x.0 > x.1 {
            x = (x.1, x.0);
        }
        if y.0 < y.1 {
            y = (y.1, y.0);
        }

        Self {
            top_left: (x.0, y.0),
            bottom_right: (x.1, y.1),
        }
    }

    fn contains(&self, (x, y): (i32, i32)) -> bool {
        x >= self.top_left.0
            && x <= self.bottom_right.0
            && y <= self.top_left.1
            && y >= self.bottom_right.1
    }

    fn is_hit_by(&self, (mut dx, mut dy): (i32, i32)) -> HitResult {
        let mut pos = (0, 0);
        let mut highest = 0;
        while pos.0 <= self.bottom_right.0 {
            pos.0 += dx;
            pos.1 += dy;
            highest = highest.max(pos.1);
            if self.contains(pos) {
                return HitResult::Hit { highest };
            }
            if dx > 0 {
                dx -= 1;
            }
            dy -= 1;
            if pos.1 < self.bottom_right.1 && dy < 0 {
                break;
            }
        }
        HitResult::Miss
    }
}

#[test]
fn test_hit() {
    let area = TargetArea {
        top_left: (20, -5),
        bottom_right: (30, -10),
    };
    dbg!(&area);
    assert_eq!(area.is_hit_by((7, 2)), HitResult::Hit { highest: 3 });
    assert_eq!(area.is_hit_by((6, 3)), HitResult::Hit { highest: 6 });
    assert_eq!(area.is_hit_by((9, 0)), HitResult::Hit { highest: 0 });
    assert_eq!(area.is_hit_by((17, -4)), HitResult::Miss);
    assert_eq!(area.is_hit_by((7, -1)), HitResult::Hit { highest: 0 });
}
#[derive(PartialEq, Debug)]
enum HitResult {
    Miss,
    Hit { highest: i32 },
}
