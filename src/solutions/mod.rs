mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
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
        5 => Box::new(day5::Solution),
        6 => Box::new(day6::Solution),
        7 => Box::new(day7::Solution),
        8 => Box::new(day8::Solution),
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
