use crate::solutions;
pub struct Solution;
use md5;

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut secret = 1;
        loop {
            let digest = md5::compute(format!("{}{}", input.trim(), secret));
            if format!("{:x}", digest).starts_with("00000") {
                break;
            }
            secret += 1;
        }
        return format!("{}", secret);
    }
    fn part2(&self, input: &str) -> String {
        let mut secret = 1;
        loop {
            let digest = md5::compute(format!("{}{}", input.trim(), secret));
            if format!("{:x}", digest).starts_with("000000") {
                break;
            }
            secret += 1;
        }
        return format!("{}", secret);
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::*;
    #[test]
    fn test_part1() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part1("abcdef"), "609043");
    }
}
*/
