use crate::solutions;
pub struct Solution;

fn read_line(line: &str) -> (usize, usize) {
    let bytes = line.trim().as_bytes();
    let code_len = bytes.len();
    let mut bytes = &bytes[1..code_len - 1];
    let mut mem_len = 0;
    while !bytes.is_empty() {
        let (ch, rest) = bytes.split_first().unwrap();
        if *ch == b'\\' {
            match rest.first() {
                Some(b'\\' | b'"') => bytes = &rest[1..],
                Some(b'x') => bytes = &rest[3..],
                Some(_) => unreachable!(),
                None => unreachable!(),
            };
        } else {
            bytes = rest;
        }
        mem_len += 1;
    }
    return (code_len, mem_len);
}

fn encode(line: &str) -> (usize, usize) {
    let bytes = line.trim().as_bytes();
    let code_len = bytes.len();
    let encoded_len = code_len + 2 + bytes.iter().filter(|&&c| c == b'"' || c == b'\\').count();
    return (code_len, encoded_len);
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let total: usize = input
            .trim()
            .lines()
            .map(read_line)
            .map(|(code, mem)| code - mem)
            .sum();
        return format!("{}", total);
    }
    fn part2(&self, input: &str) -> String {
        let total: usize = input
            .trim()
            .lines()
            .map(encode)
            .map(|(code, mem)| mem - code)
            .sum();
        return format!("{}", total);
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
        assert_eq!(
            solver.part1(
                r#"
                ""
                "abc"
                "aaa\"aaa"
                "\x27"
                "#
            ),
            "12"
        );
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(
            solver.part2(
                r#"
                ""
                "abc"
                "aaa\"aaa"
                "\x27"
                "#
            ),
            "19"
        );
    }
}
