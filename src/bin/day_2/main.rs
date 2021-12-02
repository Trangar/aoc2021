use aoc2021::{input_command_lines, Command};

fn main() {
    assert_eq!(150, dbg!(part_1(&input_command_lines!("day_2_test.txt"))));
    dbg!(part_1(&input_command_lines!("day_2_input.txt")));
    assert_eq!(900, dbg!(part_2(&input_command_lines!("day_2_test.txt"))));
    dbg!(part_2(&input_command_lines!("day_2_input.txt")));
}

fn part_1(commands: &[Command]) -> u32 {
    let mut x = 0i32;
    let mut y = 0i32;

    for command in commands {
        match *command {
            Command::Forward(n) => x += n as i32,
            Command::Down(n) => y += n as i32,
            Command::Up(n) => y -= n as i32,
            _ => {}
        }
    }

    x.unsigned_abs() * y.unsigned_abs()
}

fn part_2(commands: &[Command]) -> u32 {
    let mut aim = 0i32;
    let mut x = 0;
    let mut y = 0;

    for command in commands {
        match *command {
            Command::Forward(val) => {
                x += val;
                y += aim * val as i32;
            }
            Command::Down(val) => {
                aim += val as i32;
            }
            Command::Up(val) => {
                aim -= val as i32;
            }
            _ => {}
        }
    }

    x * y.unsigned_abs()
}
