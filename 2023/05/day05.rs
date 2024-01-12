use regex::Regex;
use std::{cmp::Ordering, fs};

const INPUT_PATH: &str = "./input.txt";

#[derive(Debug)]
struct MappingRange {
    start: isize,
    end: isize,
    delta: isize,
}

impl MappingRange {
    fn from_str(text: &str) -> Self {
        let re = Regex::new(r"(\d+) (\d+) (\d+)").expect("static regex to be valid");
        let caps = re.captures(text).unwrap();

        let start: isize = caps.get(2).unwrap().as_str().parse().unwrap();
        let len: isize = caps.get(3).unwrap().as_str().parse().unwrap();

        let end = start + len;

        let target: isize = caps.get(1).unwrap().as_str().parse().unwrap();

        let delta: isize = target - start;

        MappingRange { start, end, delta }
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn from_str(text: &str) -> Self {
        let mut ranges: Vec<MappingRange> =
            text.lines().skip(1).map(MappingRange::from_str).collect();

        ranges.sort_by(|a, b| {
            if a.start < b.start {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        Mapping { ranges }
    }

    fn transform(&self, value: isize) -> isize {
        let mapping = self
            .ranges
            .iter()
            .find(|mapping| mapping.start <= value && value <= mapping.end);

        match mapping {
            Some(mapping) => value + mapping.delta,
            None => value,
        }
    }
}

fn day_05_part_1(input: String) -> isize {
    let mut almanac = input.split("\n\n");

    let seeds = almanac.next();

    let mappings: Vec<Mapping> = almanac.map(Mapping::from_str).collect();

    seeds
        .unwrap()
        .split(' ')
        .skip(1)
        .flat_map(|val| val.parse())
        .map(|seed| {
            mappings
                .iter()
                .fold(seed, |index, mapping| mapping.transform(index))
        })
        .min()
        .unwrap()
}

fn day_05_part_2(input: String) -> isize {
    let mut almanac = input.split("\n\n");

    let seeds = almanac.next();

    let mappings: Vec<Mapping> = almanac.map(Mapping::from_str).collect();

    seeds
        .unwrap()
        .split(' ')
        .skip(1)
        .flat_map(|val| val.parse())
        .collect::<Vec<_>>()
        .chunks(2)
        .flat_map(|chunk: &[isize]| chunk[0]..=(chunk[0] + chunk[1]))
        .map(|seed| {
            mappings
                .iter()
                .fold(seed, |index, mapping| mapping.transform(index))
        })
        .min()
        .unwrap()
}

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    let res = day_05_part_2(file_content);

    println!("{}", res);
}
