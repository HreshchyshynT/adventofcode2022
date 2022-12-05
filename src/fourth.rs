use crate::utils;
use std::io::{BufRead, Result};

#[derive(Debug)]
struct IntRange {
    start: i32,
    end: i32,
}

impl From<&str> for IntRange {
    fn from(s: &str) -> Self {
        let range_vec: Vec<i32> = s.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        IntRange {
            start: range_vec[0],
            end: range_vec[1],
        }
    }
}

impl IntRange {
    fn contains(&self, other: &IntRange) -> bool {
        self.start <= other.start
            && self.start <= other.end
            && self.end >= other.start
            && self.end >= other.end
    }

    fn overlaps(&self, other: &IntRange) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (self.start <= other.end && self.end >= other.end)
    }
}

pub fn run() -> Result<()> {
    let reader = utils::read_file("./inputs/input4.txt")?;
    let mut contains_count = 0;
    let mut overlaps_count = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        let spl: Vec<IntRange> = line.split(',').map(|s| IntRange::from(s)).collect();
        if spl[0].contains(&spl[1]) || spl[1].contains(&spl[0]) {
            contains_count += 1;
        }
        if spl[0].overlaps(&spl[1]) || spl[1].overlaps(&spl[0]) {
            overlaps_count += 1;
        }
        println!("{:?}, {}", spl, contains_count,)
    }
    println!("contains count: {}", contains_count);
    println!("overlaps count: {}", overlaps_count);
    Ok(())
}
