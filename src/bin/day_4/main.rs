use std::{error::Error, fmt, str::FromStr};

fn main() {
    let test = Game::from_str(include_str!("../../../input/day_4_test.txt")).unwrap();
    let input = Game::from_str(include_str!("../../../input/day_4_input.txt")).unwrap();

    assert_eq!(4512, dbg!(part_1(test.clone())));
    assert_eq!(23177, dbg!(part_1(input.clone())));
    assert_eq!(1924, dbg!(part_2(test)));
    dbg!(part_2(input));
}

fn part_1(mut game: Game) -> u32 {
    for num in game.numbers {
        for board in &mut game.boards {
            board.remove_value(num);
            if board.is_done() {
                println!("First board to be done: {:?}", board);
                return board.unmarked_sum() * num as u32;
            }
        }
    }
    panic!("Ran out of numbers");
}

fn part_2(mut game: Game) -> u32 {
    for num in game.numbers {
        for board in &mut game.boards {
            board.remove_value(num);
        }
        if game.boards.len() == 1 && game.boards[0].is_done() {
            let board = game.boards.remove(0);
            println!("Last board: {:?}", board);
            return board.unmarked_sum() * num as u32;
        }
        game.boards.retain(|b| !b.is_done());
    }
    panic!("Ran out of numbers");
}

#[derive(Debug, Clone)]
struct Game {
    numbers: Vec<u16>,
    boards: Vec<Board>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.lines();
        let numbers = line
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut boards = Vec::new();
        let mut cells = Vec::with_capacity(25);

        for line in line {
            if line.is_empty() && !cells.is_empty() {
                boards.push(Board::new(cells.try_into().unwrap()));
                cells = Vec::with_capacity(25);
            } else {
                cells.extend(line.split_whitespace().map(|n| n.parse::<u16>().unwrap()));
            }
        }
        if !cells.is_empty() {
            boards.push(Board::new(cells.try_into().unwrap()));
        }

        Ok(Self { numbers, boards })
    }
}

#[derive(Clone)]
struct Board {
    cells: [u16; 5 * 5],
    done: [bool; 5 * 5],
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..5 {
            writeln!(f)?;
            for j in 0..5 {
                let idx = i * 5 + j;
                if self.done[idx] {
                    write!(f, "-{:02}- ", self.cells[idx])?;
                } else {
                    write!(f, " {:02}  ", self.cells[idx])?;
                }
            }
        }
        Ok(())
    }
}

impl Board {
    fn new(cells: [u16; 5 * 5]) -> Self {
        Self {
            cells,
            done: [false; 5 * 5],
        }
    }

    fn remove_value(&mut self, value: u16) {
        for (idx, val) in self.cells.iter().enumerate() {
            if val == &value {
                self.done[idx] = true;
            }
        }
    }

    fn is_done(&self) -> bool {
        for i in 0..5 {
            // check horizontal
            if self.done.iter().skip(i * 5).take(5).all(|d| *d) {
                return true;
            }

            // check vertical
            if [i, i + 5, i + 10, i + 15, i + 20]
                .iter()
                .map(|n| self.done[*n])
                .all(|d| d)
            {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> u32 {
        self.done
            .iter()
            .enumerate()
            .filter(|(_, done)| !*done)
            .map(|(idx, _)| self.cells[idx] as u32)
            .sum()
    }
}
