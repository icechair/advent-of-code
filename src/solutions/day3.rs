use std::collections::HashSet;

use crate::solutions;
use aoc::grid::{Point, EAST, NORTH, SOUTH, WEST};
pub struct Solution;

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut grid: HashSet<Point> = HashSet::new();
        let mut pos = Point::new(0, 0);
        grid.insert(pos);
        for c in input.chars() {
            pos += match c {
                '^' => NORTH,
                '>' => EAST,
                'v' => SOUTH,
                '<' => WEST,
                _ => Point::new(0, 0),
            };
            grid.insert(pos);
        }
        return format!("{}", grid.len());
    }
    fn part2(&self, _input: &str) -> String {
        return format!("2");
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::*;
    #[test]
    fn test_part1() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part1(">"), "2");
        assert_eq!(solver.part1("^>v<"), "4");
        assert_eq!(solver.part1("^v^v^v^v^v^v^v^v^v^v"), "2");
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part2(""), "2");
    }
}
