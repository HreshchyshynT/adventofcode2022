use crate::utils;
use std::{
    collections::HashSet,
    io::{BufRead, Result},
    vec,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Field {
    x: i32,
    y: i32,
}

struct Distance {
    x: i32,
    y: i32,
}

impl Distance {
    fn between(one: &Field, two: &Field) -> Self {
        let x = (one.x - two.x).abs();
        let y = (one.y - two.y).abs();
        Distance { x, y }
    }

    fn max(&self) -> i32 {
        self.x.max(self.y)
    }
}

impl Field {
    fn start() -> Self {
        Field { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Self {
        Field { x, y }
    }

    fn pull_to(&self, another: &Field) -> Self {
        let distance = Distance::between(self, another);
        if distance.max() <= 1 {
            return self.clone();
        }
        let mut x = self.x;
        let mut y = self.y;
        if another.y > self.y {
            y += 1;
        }
        if another.y < self.y {
            y -= 1;
        }
        if another.x > self.x {
            x += 1
        }
        if another.x < self.x {
            x -= 1;
        }

        Field::new(x, y)
    }
}

enum Motion {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
}

impl From<String> for Motion {
    fn from(str: String) -> Self {
        let split: Vec<&str> = str.split(' ').collect();
        let steps_count = split[1].parse().unwrap();
        match split[0] {
            "U" => Motion::Up(steps_count),
            "D" => Motion::Down(steps_count),
            "R" => Motion::Right(steps_count),
            "L" => Motion::Left(steps_count),
            _ => panic!(""),
        }
    }
}

pub fn run() -> Result<()> {
    let reader = utils::read_file("./inputs/input9.txt")?;
    let mut rope_2: Vec<Field> = vec![
        Field::start(),
        Field::start(),
        Field::start(),
        Field::start(),
        Field::start(),
        Field::start(),
        Field::start(),
        Field::start(),
        Field::start(),
        Field::start(),
    ];
    let mut rope_1: Vec<Field> = vec![Field::start(), Field::start()];
    let mut tail_his_1: HashSet<Field> = HashSet::new();
    let mut tail_his_2: HashSet<Field> = HashSet::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let motion: Motion = line.into();
        action(&mut rope_1, &mut tail_his_1, &motion);
        action(&mut rope_2, &mut tail_his_2, &motion);
    }
    println!("answer 1: {}", tail_his_1.len());
    println!("answer 2: {}", tail_his_2.len());
    Ok(())
}

fn action(rope: &mut Vec<Field>, tail_his: &mut HashSet<Field>, motion: &Motion) {
    let head_index = rope.len() - 1;
    let repeat_count = match motion {
        Motion::Down(i) => i,
        Motion::Up(i) => i,
        Motion::Left(i) => i,
        Motion::Right(i) => i,
    };
    for _ in 0..repeat_count.to_owned() {
        let current_head = rope.last().unwrap();
        let new_head = match motion {
            Motion::Down(_) => Field::new(current_head.x, current_head.y - 1),
            Motion::Up(_) => Field::new(current_head.x, current_head.y + 1),
            Motion::Left(_) => Field::new(current_head.x - 1, current_head.y),
            Motion::Right(_) => Field::new(current_head.x + 1, current_head.y),
        };
        rope[head_index] = new_head;
        for i in (0..head_index).rev() {
            let new = rope[i].pull_to(&rope[i + 1]);
            rope[i] = new;
            if i == 0 {
                tail_his.insert(rope[i]);
            }
        }
    }
}
