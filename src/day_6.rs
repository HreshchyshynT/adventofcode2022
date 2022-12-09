use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, Result},
};

use crate::utils;

#[derive(Debug)]
struct Answer {
    _line_index: usize,
    _packet_index: usize,
    _message_index: usize,
}

pub fn run() -> Result<()> {
    let reader = utils::read_file("./inputs/test6.txt")?;
    let mut results: Vec<Answer> = vec![];
    for (i, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let packet_index = first_n_distinct_characters_index(4, &line);
        let message_index = first_n_distinct_characters_index(14, &line);
        results.push(Answer {
            _line_index: i,
            _packet_index: packet_index.unwrap(),
            _message_index: message_index.unwrap(),
        })
    }
    println!("result: {results:?}");
    Ok(())
}

fn first_n_distinct_characters_index(n: usize, line: &String) -> Option<usize> {
    let mut vec_dec: VecDeque<char> = VecDeque::new();
    for (i, c) in line.chars().enumerate() {
        if vec_dec.len() == n {
            vec_dec.pop_front();
        }
        vec_dec.push_back(c);
        if vec_dec.len() == n {
            let set: HashSet<&char> = vec_dec.iter().collect();
            if set.len() == n {
                return Some(i + 1);
            }
        }
    }
    None
}
