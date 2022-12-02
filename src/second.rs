use std::{self, io::BufRead};

use crate::utils;

#[derive(PartialEq, Debug)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl HandShape {
    fn get_stronger_shape(&self) -> HandShape {
        match self {
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
            HandShape::Rock => HandShape::Paper,
        }
    }

    fn get_weeker_shape(&self) -> HandShape {
        match self {
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper,
            HandShape::Rock => HandShape::Scissors,
        }
    }

    fn beats(&self, other: &HandShape) -> bool {
        let stronger = self.get_stronger_shape();
        *other != stronger
    }

    fn play(&self, other: &HandShape) -> RoundResult {
        if self == other {
            return RoundResult::Draw;
        }

        if self.beats(other) {
            RoundResult::Win
        } else {
            RoundResult::Lose
        }
    }
}

enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl From<char> for RoundResult {
    fn from(input: char) -> Self {
        match input {
            'X' => RoundResult::Lose,
            'Y' => RoundResult::Draw,
            'Z' => RoundResult::Win,
            _ => panic!("unknown char: {}", input),
        }
    }
}

impl From<char> for HandShape {
    fn from(input: char) -> Self {
        match input {
            'A' | 'X' => HandShape::Rock,
            'B' | 'Y' => HandShape::Paper,
            'C' | 'Z' => HandShape::Scissors,
            _ => panic!("unknown char: {}", input),
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let reader = utils::read_file("./inputs/input2.txt")?;
    let mut first_total = 0;
    let mut second_total = 0;
    for line in reader.lines() {
        let round_str = line.unwrap();
        first_total += get_points_first_task(&round_str);
        second_total += get_points_second_task(&round_str);
    }
    println!("first task total points: {}", first_total);
    println!("second task total points: {}", second_total);
    Ok(())
}

fn get_points_first_task(str: &String) -> i32 {
    let chars: Vec<char> = str.chars().collect();
    let opponent: HandShape = chars[0].into();
    let my: HandShape = chars[2].into();
    let round_result = my.play(&opponent);
    my as i32 + round_result as i32
}

fn get_points_second_task(str: &String) -> i32 {
    let chars: Vec<char> = str.chars().collect();
    let opponent: HandShape = chars[0].into();
    let expected_result: RoundResult = chars[2].into();
    let my = match expected_result {
        RoundResult::Draw => opponent,
        RoundResult::Win => opponent.get_stronger_shape(),
        RoundResult::Lose => opponent.get_weeker_shape(),
    };
    my as i32 + expected_result as i32
}
