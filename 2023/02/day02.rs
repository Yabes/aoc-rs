use regex::{Captures, Regex};
use std::fs;

const INPUT_PATH: &str = "./input.txt";

#[derive(Debug)]
struct CubeSet {
    blue: u32,
    red: u32,
    green: u32,
}

impl CubeSet {
    fn from_str(input: &str) -> Self {
        let re = Regex::new(r"(\d+) (blue|red|green)").expect("Static regex to be valid");

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for (_, [count, color]) in re.captures_iter(input).map(|c| c.extract()) {
            let count: u32 = count.parse().expect("count to be a number");

            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => (),
            }
        }

        CubeSet { blue, red, green }
    }

    fn contains(&self, other_set: &CubeSet) -> bool {
        self.red >= other_set.red && self.blue >= other_set.blue && self.green >= other_set.green
    }

    fn power(set: CubeSet) -> u32 {
        set.red * set.green * set.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let re = Regex::new(r"^Game (\d+): (.*)$").expect("Static regex to be valid");
        let caps = re.captures(input).unwrap();

        let id: u32 = caps.get(1).unwrap().as_str().parse().unwrap();

        let sets = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(';')
            .map(CubeSet::from_str)
            .collect();

        Game { id, sets }
    }

    fn contains_set(&self, set: &CubeSet) -> bool {
        self.sets.iter().all(|game_set| set.contains(game_set))
    }

    fn minimal_set(&self) -> CubeSet {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for set in self.sets.iter() {
            if set.red > red {
                red = set.red;
            }

            if set.green > green {
                green = set.green;
            }

            if set.blue > blue {
                blue = set.blue;
            }
        }

        CubeSet { blue, red, green }
    }
}

fn day_02_part_1(input: String) -> u32 {
    let master_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .trim()
        .lines()
        .map(Game::from_str)
        .filter(|game| game.contains_set(&master_set))
        .map(|game| game.id)
        .sum()
}

fn day_02_part_2(input: String) -> u32 {
    let master_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .trim()
        .lines()
        .map(Game::from_str)
        .map(|game| game.minimal_set())
        .map(CubeSet::power)
        .sum()
}

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    let res = day_02_part_2(file_content);

    println!("{}", res);
}
