use std::collections::BinaryHeap;
use std::io::BufRead;

use crate::utils;

pub fn run() -> std::io::Result<()> {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let reader = utils::read_file("./inputs/input1.txt")?;
    let mut current = 0;
    for line in reader.lines() {
        let str = line.unwrap();
        if str.is_empty() {
            heap.push(current);
            current = 0;
        } else {
            let num: i32 = str.parse().unwrap();
            current += num;
        }
    }
    heap.push(current);

    println!("max: {}", heap.peek().unwrap());
    let top_3_total = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("top 3 total: {}", top_3_total);
    Ok(())
}
