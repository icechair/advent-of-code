use itertools::Itertools;

use crate::solutions;
pub struct Solution;

fn is_acending((a, b, c): (char, char, char)) -> bool {
    c > b && b > a && (c as u32) - (b as u32) == 1 && (b as u32) - (a as u32) == 1
}

fn valid_password(line: &str) -> bool {
    let pairs = line.chars().tuple_windows().filter(|(a, b)| a == b);
    if pairs.clone().count() < 2 {
        return false;
    }
    if pairs.tuple_combinations().any(|(a, b)| {
        return a == b;
    }) {
        return false;
    }
    if line.contains('i') || line.contains('o') || line.contains('l') {
        return false;
    }
    if !line.chars().tuple_windows::<(_, _, _)>().any(is_acending) {
        return false;
    }
    return true;
}

fn increment_string(line: &mut String) {
    let mut stack = String::new();
    while let Some(ch) = line.pop() {
        let mut next = ((ch as u8) + 1) as char;
        if next > 'z' {
            next = 'a'
        }
        stack.push(next);
        if next != 'a' {
            break;
        }
    }
    while let Some(ch) = stack.pop() {
        line.push(ch)
    }
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut password = input.trim().to_string();
        increment_string(&mut password);
        while !valid_password(&password) {
            increment_string(&mut password);
        }
        return format!("{}", password);
    }
    fn part2(&self, input: &str) -> String {
        let password = self.part1(input);
        let password = self.part1(&password);

        return format!("{}", password);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solver;

    #[test]
    fn test_valid_passwords() {
        assert!(!valid_password("hijklmmn"));
        assert!(!valid_password("abbceffg"));
        assert!(!valid_password("abbcegjk"));
        assert!(!valid_password("abcdefgh"));
        assert!(valid_password("abcdffaa"));
        assert!(!valid_password("ghijklmn"));
        assert!(valid_password("ghjaabcc"));

        assert!(!valid_password("abcdefhh"));
    }

    #[test]
    fn test_increment() {
        let mut line = "xyz".to_string();
        increment_string(&mut line);
        assert_eq!(line, format!("xza"));
        increment_string(&mut line);
        assert_eq!(line, format!("xzb"));
    }

    #[test]
    fn test_part1() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part1("abcdefgh"), "abcdffaa");
    }
}
