struct Grid {
    height: usize,
    width: usize,
    rows: Vec<Vec<char>>,
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn value(&self) -> (isize, isize) {
        match *self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }

    fn all_diag() -> Vec<Direction> {
        vec![
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
    }
}

struct Vec2d {
    x: isize,
    y: isize,
}

impl Vec2d {
    fn from(start_x: usize, start_y: usize, direction: &Direction, factor: isize) -> Vec2d {
        let (direction_x, direction_y) = direction.value();

        Vec2d {
            x: (direction_x * factor).wrapping_add_unsigned(start_x),
            y: (direction_y * factor).wrapping_add_unsigned(start_y),
        }
    }
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let rows: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Grid {
            height: rows.len(),
            width: rows[0].len(),
            rows,
        }
    }

    fn at(&self, pos: Vec2d) -> Option<char> {
        let x = usize::try_from(pos.x);
        let y = usize::try_from(pos.y);

        if let (Ok(x), Ok(y)) = (x, y) {
            let is_within_x = (0..self.width).contains(&x);
            let is_within_y = (0..self.height).contains(&y);

            if is_within_x && is_within_y {
                return Some(self.rows[y][x]);
            }
        }

        None
    }

    fn get_str_at(&self, x: usize, y: usize, direction: &Direction) -> Option<String> {
        let char1 = self.at(Vec2d::from(x, y, direction, 0));
        let char2 = self.at(Vec2d::from(x, y, direction, 1));
        let char3 = self.at(Vec2d::from(x, y, direction, 2));
        let char4 = self.at(Vec2d::from(x, y, direction, 3));

        match (char1, char2, char3, char4) {
            (Some(c1), Some(c2), Some(c3), Some(c4)) => Some([c1, c2, c3, c4].iter().collect()),
            _ => None,
        }
    }

    fn get_mas_str_at(&self, x: usize, y: usize, direction: &Direction) -> Option<String> {
        let char1 = self.at(Vec2d::from(x, y, direction, -1));
        let char2 = self.at(Vec2d::from(x, y, direction, 0));
        let char3 = self.at(Vec2d::from(x, y, direction, 1));

        match (char1, char2, char3) {
            (Some(c1), Some(c2), Some(c3)) => Some([c1, c2, c3].iter().collect()),
            _ => None,
        }
    }

    fn count_str_at_pos(&self, needle: &str, x: usize, y: usize) -> usize {
        Direction::all()
            .iter()
            .filter_map(|direction| self.get_str_at(x, y, direction))
            .filter(|word| word == needle)
            .count()
    }

    fn count_str(&self, needle: &str) -> usize {
        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| self.count_str_at_pos(needle, x, y))
                    .sum::<usize>()
            })
            .sum()
    }

    fn find_mas(&self, x: usize, y: usize) -> usize {
        Direction::all_diag()
            .iter()
            .filter_map(|direction| self.get_mas_str_at(x, y, direction))
            .filter(|word| word == "MAS")
            .count()
    }

    fn count_mas(&self) -> usize {
        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| if self.find_mas(x, y) == 2 { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

pub fn part1(input: &str) -> usize {
    Grid::from_str(input).count_str("XMAS")
}

pub fn part2(input: &str) -> usize {
    Grid::from_str(input).count_mas()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_grid_find_some_in_bound() {
        let expected = Some('X');

        let grid = Grid::from_str("XMAS");
        let result = grid.at(Vec2d { x: 0, y: 0 });

        assert_eq!(expected, result);
    }

    #[test]
    fn test_grid_find_none_out_of_bound() {
        let expected = None;

        let grid = Grid::from_str("XMAS");
        let result = grid.at(Vec2d { x: -1, y: 0 });

        assert_eq!(expected, result);
    }

    #[test]
    fn test_grid_get_str_at() {
        let expected = Some("XMAS");
        let grid = Grid::from_str("XMAS");
        let result = grid.get_str_at(0, 0, &Direction::East);

        assert_eq!(expected, result.as_deref());
    }

    #[test]
    fn test_grid_get_str_at_none() {
        let expected = None;
        let grid = Grid::from_str("XMAS");
        let result = grid.get_str_at(0, 0, &Direction::North);

        assert_eq!(expected, result.as_deref());
    }

    #[test]
    fn test_count_str_at_pos() {
        let expected = 8;
        let grid = Grid::from_str(
            "
S..S..S
.A.A.A.
..MMM..
SAMXMAS
..MMM..
.A.A.A.
S..S..S
",
        );

        let result = grid.count_str_at_pos("XMAS", 3, 3);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_grid_mas_str_at() {
        let expected = Some("MAS");
        let grid = Grid::from_str("XMAS");
        let result = grid.get_mas_str_at(2, 0, &Direction::East);

        assert_eq!(expected, result.as_deref());
    }

    #[test]
    fn test_find_mas() {
        let expected = 2;
        let grid = Grid::from_str(
            "
M.S
.A.
M.S
",
        );

        let result = grid.find_mas(1, 1);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part1() {
        let expected = 18;
        let result = part1(TEST_INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 9;
        let result = part2(TEST_INPUT);

        assert_eq!(expected, result);
    }
}
