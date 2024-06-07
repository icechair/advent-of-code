pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}
pub struct Solutions {
    list: Vec<Box<dyn Solution>>,
}

impl Solutions {
    pub fn new() -> Self {
        let mut list = Vec::new();
        Self { list }
    }
    pub fn solve(&self, day: usize, part: usize, input: &str) -> String {
        if part == 2 {
            return self.list[day - 1].part1(input);
        }
        return self.list[day - 1].part2(input);
    }
}
