use regex::Regex;
use rstar::{RTree, RTreeObject, AABB};
use std::fs;

const INPUT_PATH: &str = "./input.txt";

#[derive(Debug)]
struct Part {
    symbol: char,
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct PartNumber {
    value: usize,
    len: isize,
    x: isize,
    y: isize,
}

impl RTreeObject for Part {
    type Envelope = AABB<[isize; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x, self.y])
    }
}

impl RTreeObject for PartNumber {
    type Envelope = AABB<[isize; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.x, self.y], [self.x + self.len - 1, self.y])
    }
}

fn day_03_part_1(input: String) -> usize {
    let mut parts: Vec<Part> = Vec::new();
    let mut nums: Vec<PartNumber> = Vec::new();

    let re = Regex::new(r"(\d+|[^.])").expect("Static regex to be valid");

    for (index, line) in input.lines().enumerate() {
        for matched in re.captures_iter(line) {
            let matched = matched.get(1).expect("missing capture group");
            let pos_x = matched.start();
            let text = matched.as_str();
            let first_char = text.chars().next().expect("matched to be non-empty");

            if first_char.is_ascii_digit() {
                let num = PartNumber {
                    value: text.parse().expect("to parse"),
                    len: text.len() as isize,
                    x: pos_x as isize,
                    y: index as isize,
                };
                nums.push(num)
            } else {
                let part = Part {
                    symbol: first_char,
                    x: pos_x as isize,
                    y: index as isize,
                };
                parts.push(part);
            }
        }
    }

    let tree = RTree::bulk_load(parts);

    nums.iter()
        .filter(|part_number| {
            let rect = AABB::from_corners(
                [part_number.x - 1, part_number.y - 1],
                [part_number.x + part_number.len, part_number.y + 1],
            );

            let mut parts_as_pos = tree.locate_in_envelope(&rect);

            let first_part = parts_as_pos.next();
            let has_part = first_part.is_some();

            has_part
        })
        .map(|part_number| part_number.value)
        .sum()
}

fn day_03_part_2(input: String) -> usize {
    let mut parts: Vec<Part> = Vec::new();
    let mut nums: Vec<PartNumber> = Vec::new();

    let re = Regex::new(r"(\d+|[^.])").expect("Static regex to be valid");

    for (index, line) in input.lines().enumerate() {
        for matched in re.captures_iter(line) {
            let matched = matched.get(1).expect("missing capture group");
            let pos_x = matched.start();
            let text = matched.as_str();
            let first_char = text.chars().next().expect("matched to be non-empty");

            if first_char.is_ascii_digit() {
                let num = PartNumber {
                    value: text.parse().expect("to parse"),
                    len: text.len() as isize,
                    x: pos_x as isize,
                    y: index as isize,
                };
                nums.push(num)
            } else if first_char == '*' {
                let part = Part {
                    symbol: first_char,
                    x: pos_x as isize,
                    y: index as isize,
                };
                parts.push(part);
            }
        }
    }

    let tree = RTree::bulk_load(nums);

    parts
        .iter()
        .filter_map(|part| {
            let rect = AABB::from_corners([part.x - 1, part.y - 1], [part.x + 1, part.y + 1]);

            let mut part_numbers = tree.locate_in_envelope_intersecting(&rect);

            let first = part_numbers.next();
            let second = part_numbers.next();

            match (first, second) {
                (Some(n1), Some(n2)) => Some(n1.value * n2.value),
                _ => None,
            }
        })
        .sum()
}

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    let res = day_03_part_2(file_content);

    println!("{}", res);
}
