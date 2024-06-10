mod day1;
mod day2;
mod day3;
mod day4;
pub trait Solver {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

fn get_solver(day: usize) -> Box<dyn Solver> {
    match day {
        1 => Box::new(day1::Solution),
        2 => Box::new(day2::Solution),
        3 => Box::new(day3::Solution),
        4 => Box::new(day4::Solution),
        _ => unimplemented!(),
    }
}

pub fn solve(day: usize, part: usize, input: &str) -> String {
    let solver = get_solver(day);
    if part == 2 {
        return solver.part2(input);
    }
    return solver.part1(input);
}
