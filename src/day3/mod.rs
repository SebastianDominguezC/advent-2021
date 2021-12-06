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
    println!("Day 3");
    let vec = read(File::open("src/day3/input.txt").unwrap()).unwrap();

    let counts = get_counts(&vec);

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for i in 0..counts.len() {
        if counts[i].0 > counts[i].1 {
            gamma = gamma + "0";
            epsilon = epsilon + "1";
        } else {
            gamma = gamma + "1";
            epsilon = epsilon + "0";
        }
    }

    let gamma = isize::from_str_radix(&gamma[..], 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon[..], 2).unwrap();
    let power = gamma * epsilon;
    println!("gamma: {} * epsilon: {} = power: {}", gamma, epsilon, power);

    // Part 2
    let o = oxygen_filter(vec.clone(), 0);
    let c = carbon_filter(vec, 0);

    let o = isize::from_str_radix(&o[0][..], 2).unwrap();
    let c = isize::from_str_radix(&c[0][..], 2).unwrap();
    println!("Oxygen levels: {:?}", o);
    println!("Carbon levels: {:?}", c);
    println!("Life support: {:?}", o * c);
}

fn oxygen_filter(info: Vec<String>, i: usize) -> Vec<String> {
    let counts = get_counts(&info);
    let num = if counts[i].0 > counts[i].1 { '0' } else { '1' };
    let filter: Vec<String> = info
        .iter()
        .filter(|v| v.as_bytes()[i] as char == num)
        .map(|v| v.clone())
        .collect();

    if filter.len() <= 1 {
        filter
    } else {
        oxygen_filter(filter, i + 1)
    }
}

fn carbon_filter(info: Vec<String>, i: usize) -> Vec<String> {
    let counts = get_counts(&info);
    let num = if counts[i].0 > counts[i].1 { '1' } else { '0' };
    let filter: Vec<String> = info
        .iter()
        .filter(|v| v.as_bytes()[i] as char == num)
        .map(|v| v.clone())
        .collect();

    if filter.len() <= 1 {
        filter
    } else {
        carbon_filter(filter, i + 1)
    }
}

fn get_counts(info: &Vec<String>) -> Vec<(i32, i32)> {
    let l = info[0].len();
    let mut counts = vec![(0, 0); l];
    for bin in info.iter() {
        for (j, n) in bin.chars().enumerate() {
            if n == '0' {
                counts[j].0 += 1;
            } else if n == '1' {
                counts[j].1 += 1;
            }
        }
    }
    counts
}
