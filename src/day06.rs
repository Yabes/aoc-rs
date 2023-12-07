use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "./input.txt";

fn day_06_part_1(input: String) -> isize {
    let re = Regex::new(r"\d+").expect("static regex to be valid");

    let mut lines = input.lines().map(|line| {
        re.find_iter(line)
            .flat_map(|token| token.as_str().parse::<isize>())
    });

    let races: Vec<(isize, isize)> = lines.next().unwrap().zip(lines.next().unwrap()).collect();

    races
        .iter()
        .map(|(time, distance)| {
            (1..*time)
                .filter(|pressed_duration| {
                    let rest = time - pressed_duration;

                    pressed_duration * rest > *distance
                })
                .count() as isize
        })
        .product()
}

fn day_06_part_2(input: String) -> isize {
    let re = Regex::new(r"\d+").expect("static regex to be valid");

    let input = input.replace(' ', "");

    let mut lines = input.lines().map(|line| {
        re.find_iter(line)
            .flat_map(|token| token.as_str().parse::<isize>())
    });

    let races: Vec<(isize, isize)> = lines.next().unwrap().zip(lines.next().unwrap()).collect();

    races
        .iter()
        .map(|(time, distance)| {
            (1..*time)
                .filter(|pressed_duration| {
                    let rest = time - pressed_duration;

                    pressed_duration * rest > *distance
                })
                .count() as isize
        })
        .product()
}

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    let res = day_06_part_2(file_content);

    println!("{}", res);
}
