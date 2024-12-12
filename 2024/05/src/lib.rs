struct Graph {
    nodes: Vec<Node>,
}

struct Node {
    value: usize,
    edges: Vec<Node>,
}

struct Book {
    pages: Vec<usize>
}

impl Graph {
    fn from_str(input: &str) -> Self {}
}

impl Node {
    fn from_str(input: &str) -> Self {}
}

impl Book {
    fn from_str(input: &str) -> Self {}

    fn get_midpage() -> usize;
}

pub fn part1(input: &str) -> usize {
    unimplemented!();
}

pub fn part2(input: &str) -> usize {
    unimplemented!();
}

fn parse_input(input: &str) -> impl Iterator<Item = ()> + use<'_> {
    input.trim().lines().filter_map(|line| {
        unimplemented!();
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_parse() {
        let expected = vec![];
        let result = parse_input(TEST_INPUT).collect();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part1() {
        let expected = 0;
        let result = part1(TEST_INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let result = part2(TEST_INPUT);

        assert_eq!(expected, result);
    }
}
