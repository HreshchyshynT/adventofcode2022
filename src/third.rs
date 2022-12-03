use std::collections::HashSet;
use std::io::BufRead;

use crate::utils;

pub fn run() -> std::io::Result<()> {
    let reader = utils::read_file("./inputs/input3.txt")?;
    let mut bags: Vec<HashSet<char>> = vec![];
    let mut total: isize = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        bags.push(line.chars().collect::<HashSet<char>>());
        if bags.len() == 3 {
            match find_badge(&bags[0], &bags[1], &bags[2]) {
                Some(c) => total += c.get_points(),
                None => panic!("badge absent"),
            }
            bags.clear();
        }
    }
    println!("result: {:?}", total);
    Ok(())
}

trait HasPoints {
    fn get_points(&self) -> isize;
}

impl HasPoints for char {
    fn get_points(&self) -> isize {
        let c = self.clone();
        match c.is_lowercase() {
            true => 1 + c as isize - 'a' as isize,
            false => 27 + c as isize - 'A' as isize,
        }
    }
}

fn find_badge(bag1: &HashSet<char>, bag2: &HashSet<char>, bag3: &HashSet<char>) -> Option<char> {
    for c in bag3.iter() {
        if bag1.contains(&c) && bag2.contains(&c) {
            return Some(*c);
        }
    }
    return None;
}
