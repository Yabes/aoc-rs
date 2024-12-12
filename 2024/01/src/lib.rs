use std::cmp::Ordering;

pub fn part1(input: &str) -> usize {
    let parsed = parse_input(input);

    let (mut left, mut right): (Vec<usize>, Vec<usize>) = parsed.unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| {
            if left < right {
                right - left
            } else {
                left - right
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let parsed = parse_input(input);

    let (mut left, mut right): (Vec<usize>, Vec<usize>) = parsed.unzip();

    left.sort();
    right.sort();

    left.iter()
        .map(|val| {
            let mut count = 0;

            for target in &right {
                match val.cmp(target) {
                    Ordering::Less => break,
                    Ordering::Equal => count += 1,
                    Ordering::Greater => (),
                }
            }

            count * val
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    input.lines().filter_map(|line| {
        let nums: Vec<usize> = line
            .split("   ")
            .filter_map(|str| str.parse().ok())
            .collect();

        let first = nums.first();
        let last = nums.last();

        match (first, last) {
            (Some(first), Some(last)) => Some((*first, *last)),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_parse() {
        let expected = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        let result: Vec<(usize, usize)> = parse_input(TEST_INPUT).collect();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part1() {
        let expected = 11;
        let result = part1(TEST_INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 31;
        let result = part2(TEST_INPUT);

        assert_eq!(expected, result);
    }
}
