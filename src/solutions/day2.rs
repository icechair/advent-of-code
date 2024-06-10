use crate::solutions;
pub struct Solution;

fn surface_area(l: usize, w: usize, h: usize) -> usize {
    return (2 * l * w) + (2 * w * h) + (2 * h * l);
}

fn line_to_dimensions(line: &str) -> (usize, usize, usize) {
    let mut parts = line.split('x');
    let l = parts.next().unwrap().parse().unwrap();
    let w = parts.next().unwrap().parse().unwrap();
    let h = parts.next().unwrap().parse().unwrap();
    return (l, w, h);
}

fn perimeter(l: usize, w: usize, h: usize) -> usize {
    let a = usize::min(l, w);
    let a = usize::min(a, h);
    let b = match a {
        x if x == l => usize::min(w, h),
        x if x == w => usize::min(l, h),
        x if x == h => usize::min(w, l),
        _ => unreachable!(),
    };
    return 2 * a + 2 * b;
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut area = 0;
        for line in input.lines() {
            let (l, w, h) = line_to_dimensions(line);
            let slack = usize::min(l * w, l * h);
            let slack = usize::min(slack, w * h);
            area += surface_area(l, w, h) + slack;
        }
        return format!("{}", area);
    }
    fn part2(&self, input: &str) -> String {
        let mut ribbon = 0;
        for line in input.lines() {
            let (l, w, h) = line_to_dimensions(line);
            let peri = perimeter(l, w, h);
            ribbon += peri + (w * l * h);
        }
        return format!("{}", ribbon);
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
        assert_eq!(solver.part1("2x3x4"), "58");
        assert_eq!(solver.part1("1x1x10"), "43");
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        assert_eq!(solver.part2("2x3x4"), "34");
        assert_eq!(solver.part2("1x1x10"), "14");
    }
}
