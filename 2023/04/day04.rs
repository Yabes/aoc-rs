use std::{collections::HashMap, fs};

use regex::Regex;

const INPUT_PATH: &str = "./input.txt";

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<i32>,
    player_numbers: Vec<i32>,
}

fn str_to_vec(text: &str) -> Vec<i32> {
    text.split(' ').flat_map(|n| n.parse()).collect()
}

impl Card {
    fn from_str(text: &str) -> Self {
        let re = Regex::new(r"Card\s+(\d+): (.*) \| (.*)").expect("static regex to be valid");
        let caps = re.captures(text).unwrap();

        let id = caps.get(1).unwrap().as_str().parse().unwrap();

        let winning_numbers = caps.get(2).unwrap().as_str();
        let winning_numbers = str_to_vec(winning_numbers);

        let player_numbers = caps.get(3).unwrap().as_str();
        let player_numbers = str_to_vec(player_numbers);

        Card {
            id,
            winning_numbers,
            player_numbers,
        }
    }
}

fn day_04_part_1(input: String) -> u32 {
    input
        .trim()
        .lines()
        .map(Card::from_str)
        .map(|c| {
            let count = c
                .player_numbers
                .iter()
                .filter(|player_number| c.winning_numbers.contains(player_number))
                .count();

            match count {
                0 => 0,
                1 => 1,
                x => 2_u32.pow(x as u32 - 1),
            }
        })
        .sum()
}

fn day_04_part_2(input: String) -> u32 {
    let mut card_counts: HashMap<u32, u32> = HashMap::new();

    input.trim().lines().map(Card::from_str).for_each(|c| {
        let wins = c
            .player_numbers
            .iter()
            .filter(|player_number| c.winning_numbers.contains(player_number))
            .count();

        let count = *card_counts.entry(c.id).or_insert(1);

        for delta in 1..=wins {
            let index = c.id + delta as u32;
            let next_count = card_counts.entry(index).or_insert(1);
            *next_count += count;
        }
    });

    card_counts.values().sum()
}

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    let res = day_04_part_2(file_content);

    println!("{}", res);
}
