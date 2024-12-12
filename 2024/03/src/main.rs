use aoc_2024_03::{part1, part2};
use std::fs;

const INPUT_PATH: &str = "./input.txt";

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    println!("Part 1: {}", part1(&file_content));
    println!("Part 2: {}", part2(&file_content));
}
