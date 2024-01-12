use aoc_2023_07::{part1, part2};
use std::fs;

const INPUT_PATH: &str = "./input.txt";

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    if let Ok(value) = part1(&file_content) {
        println!("Part 1: {}", value);
    }

    if let Ok(value) = part2(&file_content) {
        println!("Part 2: {}", value);
    }
}
