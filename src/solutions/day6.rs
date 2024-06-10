use crate::solutions;
pub struct Solution;

const WIDTH: usize = 1000;

fn parse_cmd(line: &str) -> (&str, usize, usize, usize, usize) {
    let mut tokens = line.split(|c| c == ' ' || c == ',');
    let mut cmd = tokens.next().unwrap();
    if cmd == "turn" {
        cmd = tokens.next().unwrap();
    }
    let l: usize = tokens.next().unwrap().parse().unwrap();
    let t: usize = tokens.next().unwrap().parse().unwrap();
    tokens.next(); //through
    let r: usize = tokens.next().unwrap().parse().unwrap();
    let b: usize = tokens.next().unwrap().parse().unwrap();
    return (cmd, l, t, r, b);
}

fn get_index(row: usize, col: usize, width: usize) -> usize {
    (row * width) + col
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut lights = [0u8; WIDTH * WIDTH];
        for line in input.lines() {
            let (cmd, left, top, right, bottom) = parse_cmd(line);
            for row in top..=bottom {
                for col in left..=right {
                    let idx = get_index(row, col, WIDTH);
                    lights[idx] = match cmd {
                        "on" => 1,
                        "off" => 0,
                        "toggle" => {
                            if lights[idx] == 1 {
                                0
                            } else {
                                1
                            }
                        }
                        _ => 0,
                    }
                }
            }
        }
        return format!("{}", lights.iter().filter(|&&x| x > 0).count());
    }

    fn part2(&self, input: &str) -> String {
        let mut lights = [0u8; WIDTH * WIDTH];
        for line in input.lines() {
            let (cmd, left, top, right, bottom) = parse_cmd(line);
            for row in top..=bottom {
                for col in left..=right {
                    let idx = get_index(row, col, WIDTH);
                    lights[idx] = match cmd {
                        "on" => lights[idx] + 1,
                        "off" => {
                            if lights[idx] > 0 {
                                lights[idx] - 1
                            } else {
                                0
                            }
                        }
                        "toggle" => lights[idx] + 2,
                        _ => 0,
                    }
                }
            }
        }
        return format!("{}", lights.iter().fold(0, |acc, &c| acc + usize::from(c)));
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
        assert_eq!(solver.part1("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500"), "998996");
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(
            solver.part2("turn on 0,0 through 0,0\ntoggle 0,0 through 999,999"),
            "2000001"
        );
    }
}
