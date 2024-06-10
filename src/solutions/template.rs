use crate::solutions;
pub struct Solution;

impl solutions::Solver for Solution {
    fn part1(&self, _input: &str) -> String {
        return format!("1");
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
        assert_eq!(solver.part1(""), "1");
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part2(""), "2");
    }
}
