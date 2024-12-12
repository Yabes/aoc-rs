pub fn part1(input: &str) -> usize {
    parse_input(input).filter(is_valid).count()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .filter(|levels| {
            let is_valid_as_is = is_valid(levels);

            if is_valid_as_is {
                return is_valid_as_is;
            }

            for i in 0..levels.len() {
                let mut small = levels.clone();
                small.remove(i);

                let is_small_valid = is_valid(&small);

                if is_small_valid {
                    return is_small_valid
                }
            }

            false
        })
        .count()
}

fn is_valid(levels: &Vec<usize>) -> bool {
    let mut diffs = levels.windows(2).map(|w| w[0].abs_diff(w[1]));

    let ordered = levels.is_sorted() || levels.iter().rev().is_sorted();
    let low_increase = diffs.all(|diff| (1..=3).contains(&diff));

    ordered && low_increase
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<usize>> + use<'_> {
    input.trim().lines().map(|line| {
        line.split(" ")
            .map(|num| num.parse::<usize>().expect("Number to be valid"))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse() {
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let result: Vec<Vec<usize>> = parse_input(TEST_INPUT).collect();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part1() {
        let expected = 2;
        let result = part1(TEST_INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 4;
        let result = part2(TEST_INPUT);

        assert_eq!(expected, result);
    }
}
