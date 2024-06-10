use std::collections::HashSet;

use crate::solutions;
use aoc::grid::{Point, EAST, NORTH, SOUTH, WEST};
pub struct Solution;

fn make_move(p: Point, c: char) -> Point {
    p + match c {
        '^' => NORTH,
        '>' => EAST,
        'v' => SOUTH,
        '<' => WEST,
        _ => Point::new(0, 0),
    }
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut grid: HashSet<Point> = HashSet::new();
        let mut pos = Point::new(0, 0);
        grid.insert(pos);
        for c in input.chars() {
            pos = make_move(pos, c);
            grid.insert(pos);
        }
        return format!("{}", grid.len());
    }
    fn part2(&self, input: &str) -> String {
        let mut grid: HashSet<Point> = HashSet::new();
        let mut pos_santa = Point::new(0, 0);
        let mut pos_robo = Point::new(0, 0);
        grid.insert(pos_santa);
        for (i, c) in input.chars().enumerate() {
            if i % 2 == 0 {
                pos_santa = make_move(pos_santa, c);
                grid.insert(pos_santa);
            } else {
                pos_robo = make_move(pos_robo, c);
                grid.insert(pos_robo);
            }
        }
        return format!("{}", grid.len());
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
        assert_eq!(solver.part2("^v"), "3");
        assert_eq!(solver.part2("^>v<"), "3");
        assert_eq!(solver.part2("^v^v^v^v^v"), "11");
    }
}
