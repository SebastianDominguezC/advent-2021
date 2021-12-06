use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn main() {
    // Part 1
    println!("Day 1");
    let vec = read(File::open("src/day1/input.txt").unwrap()).unwrap();
    let mut counter = 0;

    for i in 1..vec.len() {
        if vec[i] > vec[i - 1] {
            counter += 1;
        }
    }

    println!("{}", counter);

    // Part 2
    let mut counter = 0;

    let mut previous;
    let mut current;

    previous = vec[0] + vec[0 + 1] + vec[0 + 2];

    for i in 1..vec.len() - 2 {
        current = vec[i] + vec[i + 1] + vec[i + 2];
        if current > previous {
            counter += 1;
        }
        previous = current;
    }

    println!("{}", counter);
}
