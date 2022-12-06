use core::result::Result as CoreResult;
use std::{
    collections::HashMap,
    collections::LinkedList,
    io::{BufRead, Result},
};

use crate::utils;
pub fn run() -> Result<()> {
    let reader = utils::read_file("./inputs/input5.txt")?;
    let mut ord_stack_map: HashMap<usize, LinkedList<char>> = HashMap::new();
    // stack index to line index
    let mut indecies_map: HashMap<u32, usize> = HashMap::new();

    for line in reader.lines().map(CoreResult::unwrap) {
        if line.contains('[') {
            for (index, char) in line.char_indices() {
                if char.is_alphabetic() {
                    if let Some(list) = ord_stack_map.get_mut(&index) {
                        list.push_back(char);
                    } else {
                        let mut list: LinkedList<char> = LinkedList::new();
                        list.push_back(char);
                        ord_stack_map.insert(index, list);
                    }
                }
            }
        } else if line.contains("move") {
            println!("stacks: {:?}", ord_stack_map);
            let split: Vec<&str> = line.split(' ').collect();
            let quantity: u32 = split[1].parse().unwrap();
            let from: u32 = split[3].parse().unwrap();
            let to: u32 = split[5].parse().unwrap();
            println!("move {} from {} to {}", quantity, from, to);
            let from_stack_line_index = indecies_map[&from];
            let to_stack_line_index = indecies_map[&to];
            let stack = ord_stack_map.get_mut(&from_stack_line_index).unwrap();
            let mut vec: Vec<char> = Vec::new();
            for _ in 0..quantity {
                match stack.pop_front() {
                    Some(n) => vec.push(n),
                    None => {}
                };
            }
            let stack = ord_stack_map.get_mut(&to_stack_line_index).unwrap();
            vec.reverse();
            for c in vec {
                stack.push_front(c);
            }
        } else if line.contains('1') {
            for (index, char) in line.char_indices() {
                if char.is_digit(10) {
                    let i = char.to_digit(10).unwrap();
                    indecies_map.insert(i, index);
                }
            }
        }
    }
    println!("stacks: {:?}", ord_stack_map);
    for entry in ord_stack_map.iter() {
        println!("{}: {}", entry.0, entry.1.front().unwrap())
    }
    Ok(())
}
