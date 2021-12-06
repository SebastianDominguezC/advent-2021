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
    println!("Day 2");
    let vec = read(File::open("src/day2/input.txt").unwrap()).unwrap();
    let mut x = 0;
    let mut y = 0;
    for command in vec.iter() {
        let (dir, units) = command.split_at(command.find(" ").unwrap());
        let units: i32 = units.trim().parse().unwrap();

        if dir == "forward" {
            x += units;
        } else if dir == "down" {
            y += units;
        } else if dir == "up" {
            y -= units;
        }
    }
    println!("x: {}, y:{}", x, y);
    println!("x * y = {}", x * y);

    // Part 2
    let mut x = 0;
    let mut y: i64 = 0;
    let mut aim = 0;
    for command in vec.iter() {
        let (dir, units) = command.split_at(command.find(" ").unwrap());
        let units: i32 = units.trim().parse().unwrap();

        if dir == "forward" {
            x += units;
            y += (aim * units) as i64;
        } else if dir == "down" {
            aim += units;
        } else if dir == "up" {
            aim -= units;
        }
    }
    println!("x: {}, y: {}, aim:{}", x, y, aim);
    println!("x * y = {}", x as i64 * y);
}
