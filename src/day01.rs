use regex::{Captures, Regex};

fn day_01_part_1(input: String) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let chars: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();

            let first = chars.first();
            let last = chars.last();

            match (first, last) {
                (Some(first), Some(last)) => Some(format!("{}{}", first, last)),
                _ => None,
            }
        })
        .flat_map(|token| token.parse::<u32>())
        .sum()
}

fn day_01_part_2(input: String) -> u32 {
    let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let replacement = |caps: &Captures| -> String {
        match &caps[0] {
            "one" => "o1e",
            "two" => "t2o",
            "three" => "t3e",
            "four" => "f4r",
            "five" => "f5e",
            "six" => "s6x",
            "seven" => "s7n",
            "eight" => "e8t",
            "nine" => "n9e",
            _ => panic!("bad capture"),
        }
        .to_string()
    };

    input
        .lines()
        .filter_map(|line| {
            let line = re.replace_all(line, replacement);
            let line = re.replace_all(&line, replacement);

            let chars: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();

            let first = chars.first();
            let last = chars.last();

            match (first, last) {
                (Some(first), Some(last)) => Some(format!("{}{}", first, last)),
                _ => None,
            }
        })
        .flat_map(|token| token.parse::<u32>())
        .sum()
}
