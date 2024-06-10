use crate::solutions;
pub struct Solution;

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut floor = 0;
        for c in input.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => unreachable!(),
            }
        }
        return format!("{}", floor);
    }

    fn part2(&self, input: &str) -> String {
        let mut floor = 0;
        let mut n = 0;
        for c in input.chars() {
            n += 1;
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => unreachable!(),
            }
            if floor < 0 {
                return format!("{}", n);
            }
        }
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
        assert_eq!(solver.part1("(())"), "0");
        assert_eq!(solver.part1("()()"), "0");
        assert_eq!(solver.part1("((("), "3");
        assert_eq!(solver.part1("(()(()("), "3");
        assert_eq!(solver.part1("))((((("), "3"); // also results in floor 3.
        assert_eq!(solver.part1("())"), "-1");
        assert_eq!(solver.part1("))("), "-1"); // both result in floor -1 (the first basement level).
        assert_eq!(solver.part1(")))"), "-3");
        assert_eq!(solver.part1(")())())"), "-3"); // both result in floor -3.
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part2(")"), "1");
        assert_eq!(solver.part2("()())"), "5");
    }
}
