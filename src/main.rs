use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "./input.txt";

#[derive(Debug)]
struct CustomStruct {
    id: isize,
}

impl CustomStruct {
    fn from_str(text: &str) -> Self {
        let re = Regex::new(r"").expect("static regex to be valid");

        CustomStruct { id: 0 }
    }
}

fn day_0x_part_1(input: String) -> isize {
    unimplemented!();
}

fn day_0x_part_2(input: String) -> isize {
    unimplemented!();
}

fn main() {
    let file_content = fs::read_to_string(INPUT_PATH).expect("Cannot open input.txt");

    let res = day_0x_part_1(file_content);

    println!("{}", res);
}
