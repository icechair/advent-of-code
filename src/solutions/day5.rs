use itertools::Itertools;

use crate::solutions;
pub struct Solution;

fn string_is_nice(line: &str) -> bool {
    line.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
        && !(["ab", "cd", "pq", "xy"].iter().any(|x| line.contains(x)))
        && line.chars().tuple_windows().any(|(a, b)| a == b)
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut amount = 0;
        for line in input.lines() {
            if string_is_nice(line) {
                amount += 1;
            }
        }
        return format!("{}", amount);
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
        assert_eq!(solver.part1("ugknbfddgicrmopn"), "1");
        assert_eq!(solver.part1("aaa"), "1");
        assert_eq!(solver.part1("jchzalrnumimnmhp"), "0");
        assert_eq!(solver.part1("haegwjzuvuyypxyu"), "0");
        assert_eq!(solver.part1("dvszwmarrgswjxmb"), "0");
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part2(""), "2");
    }
}