use regex::{Regex};

pub fn part1(input: &str) -> usize {
    parse_input(input).iter().map(|(left, right)| {
        left * right
    }).sum()
}

pub fn part2(input: &str) -> usize {
  let re = Regex::new(r"don't\(\).*?do\(\)").expect("Static regex to be valid");

    let replace = input.replace("\n", "");
    let input_clean = re.replace_all(&replace, "");
    parse_input(&input_clean).iter().map(|(left, right)| {
        left * right
    }).sum()
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
  let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Static regex to be valid");

  re.captures_iter(input).map(|c| c.extract()).map(|(_, [x, y]) | {
    (x.parse().expect("valid number"), y.parse().expect("valid number"))
  }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

";

    #[test]
    fn test_parse() {
        let expected = vec![(2,4), (5,5), (11,8), (8,5)];
        let result = parse_input(TEST_INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part1() {
        let expected = 161;
        let result = part1(TEST_INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 48;
        let result = part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()do()");

        assert_eq!(expected, result);
    }
}
