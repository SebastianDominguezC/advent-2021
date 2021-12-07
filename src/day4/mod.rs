use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn main() {
    // Part 1
    println!("Day 4");
    let mut vec = read(File::open("src/day4/input.txt").unwrap()).unwrap();
    let draws = vec.remove(0);

    let draws: Vec<i32> = draws.split(",").map(|d| d.parse().unwrap()).collect();

    vec.remove(0);
    let mut boards = vec![];
    let mut next_board = Board::new();
    for row in vec.iter() {
        if row == "" {
            boards.push(next_board);
            next_board = Board::new();
        } else {
            next_board.add_row(row);
        }
    }

    let mut winner = Board::new();
    let mut win_num = -1;

    for num in draws.iter() {
        let mut has_winner = false;
        for board in &mut boards.iter_mut() {
            board.check_number(*num);
            if board.check_win() {
                winner = board.clone();
                has_winner = true;
                win_num = *num;
                break;
            }
        }
        if has_winner {
            break;
        };
    }

    let sum = winner.sum_unmarked();
    let mult = win_num * sum;
    println!("{:?}", sum);
    println!("{:?}", mult);

    // Part 2
    println!("Part 2 -------------");
    let mut vec = read(File::open("src/day4/input.txt").unwrap()).unwrap();
    let draws = vec.remove(0);

    let draws: Vec<i32> = draws.split(",").map(|d| d.parse().unwrap()).collect();

    vec.remove(0);
    let mut boards = vec![];
    let mut next_board = Board::new();
    for row in vec.iter() {
        if row == "" {
            boards.push(next_board);
            next_board = Board::new();
        } else {
            next_board.add_row(row);
        }
    }

    let mut winners = boards.clone();
    let mut last_winner = Board::new();
    let mut win_num = -1;

    let mut i = 0;
    while winners.len() > 0 {
        let num = draws[i];

        for (i, board) in &mut winners.iter_mut().enumerate() {
            board.check_number(num);
            if board.check_win() {
                board.remove = true;
                last_winner = board.clone();
                win_num = num;
            }
        }
        winners = winners
            .iter()
            .filter(|v| !v.remove)
            .map(|v| v.clone())
            .collect();
        i += 1;
    }
    let sum = last_winner.sum_unmarked();
    let mult = win_num * sum;
    println!("{:?}", sum);
    println!("{:?}", mult);
}

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<i32>>,
    remove: bool,
}

impl Board {
    fn new() -> Self {
        Self {
            grid: vec![],
            remove: false,
        }
    }
    fn add_row(&mut self, row: &String) {
        let row: Vec<i32> = row
            .trim()
            .split(" ")
            .filter(|v| v != &"")
            .map(|d| d.trim().parse().unwrap())
            .collect();
        self.grid.push(row);
    }

    fn check_number(&mut self, num: i32) {
        self.grid = self
            .grid
            .iter()
            .map(|v| v.iter().map(|a| if a == &num { -1 } else { *a }).collect())
            .collect();
    }

    fn check_win(&mut self) -> bool {
        // check if entire row is == -1
        for row in self.grid.iter() {
            let mut win = true;
            for num in row.iter() {
                win = win && num == &-1;
            }
            if win {
                return true;
            }
        }
        // check if columns == -1
        for i in 0..self.grid.len() {
            let mut win = true;
            for j in 0..self.grid.len() {
                // i = col, j == row
                win = win && self.grid[j][i] == -1;
            }
            if win {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&mut self) -> i32 {
        let mut nums = vec![];
        for row in self.grid.iter() {
            for num in row.iter() {
                if num != &-1 {
                    nums.push(*num)
                }
            }
        }
        nums.iter().sum()
    }
}
