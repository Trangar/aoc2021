fn main() {
    let test = SchoolOfFish::new(include_str!("../../../input/day_6_test.txt"));
    let input = SchoolOfFish::new(include_str!("../../../input/day_6_input.txt"));

    assert_eq!(5934, dbg!(part_1(test.clone())));
    assert_eq!(362666, dbg!(part_1(input.clone())));
    assert_eq!(26984457539, dbg!(part_2(test)));
    dbg!(part_2(input));
}

#[derive(Clone, Debug)]
struct SchoolOfFish {
    pub ages: [usize; 9],
}

impl SchoolOfFish {
    pub fn new(input: &str) -> Self {
        let mut result = Self { ages: [0; 9] };
        for n in input.split(',').map(|f| f.trim().parse::<usize>().unwrap()) {
            result.ages[n] += 1;
        }
        result
    }

    pub fn next_age(&mut self) {
        let age_0 = self.ages[0];
        for i in 1..self.ages.len() {
            self.ages[i - 1] = self.ages[i];
        }
        self.ages[6] += age_0;
        self.ages[8] = age_0;
    }
}

fn part_1(mut school: SchoolOfFish) -> usize {
    for _ in 0..80 {
        school.next_age();
    }
    school.ages.into_iter().sum()
}

fn part_2(mut school: SchoolOfFish) -> usize {
    for _ in 0..256 {
        school.next_age();
    }
    school.ages.into_iter().sum()
}
