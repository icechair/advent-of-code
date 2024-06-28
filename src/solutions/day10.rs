use crate::solutions;
use itertools::Itertools;

pub struct Solution;

fn look_and_say(line: &str) -> String {
    if line.len() == 1 {
        return format!("1{}", line);
    }
    let mut next = String::new();
    let mut prev_ch = '?';
    let mut count = 0;
    for (i, ch) in line.chars().enumerate() {
        if i == 0 {
            prev_ch = ch;
            count = 1;
            continue;
        }
        if prev_ch == ch {
            count += 1;
            continue;
        }
        next += &format!("{}{}", count, prev_ch);
        prev_ch = ch;
        count = 1;
    }
    if count > 0 {
        next += &format!("{}{}", count, prev_ch);
    }
    return next;
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut line = input.trim().to_string();
        for _ in 0..40 {
            line = look_and_say(&line);
        }
        return format!("{}", line.len());
    }
    fn part2(&self, input: &str) -> String {
        let mut line = input.trim().to_string();
        for _ in 0..50 {
            line = look_and_say(&line);
        }
        return format!("{}", line.len());
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solver;

    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(look_and_say("1"), format!("11"));
        assert_eq!(look_and_say("11"), format!("21"));
        assert_eq!(look_and_say("21"), format!("1211"));
        assert_eq!(look_and_say("1211"), format!("111221"));
        assert_eq!(look_and_say("111221"), format!("312211"));
    }
}
