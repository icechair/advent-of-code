use std::collections::HashMap;

use crate::solutions;
pub struct Solution;

#[derive(Eq, PartialEq, Debug)]
struct Distance<'a> {
    node: &'a str,
    distance: usize,
}

struct Graph<'a> {
    nodes: HashMap<&'a str, Vec<Distance<'a>>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
    pub fn insert(&mut self, line: &'a str) {
        let mut tokens = line.trim().split(' ');
        let left = tokens.next().unwrap();
        tokens.next();
        let right = tokens.next().unwrap();
        tokens.next();
        let distance = tokens.next().unwrap().parse::<usize>().unwrap();
        let nd = Distance {
            node: right,
            distance,
        };
        let item = self.nodes.entry(left).or_insert(Vec::new());
        item.push(nd);

        let nd = Distance {
            node: left,
            distance,
        };
        let item = self.nodes.entry(right).or_insert(Vec::new());
        item.push(nd);
    }

    pub fn sort_min(&mut self) {
        for (_, v) in self.nodes.iter_mut() {
            v.sort_by(|a, b| a.distance.cmp(&b.distance));
        }
    }
    pub fn sort_max(&mut self) {
        for (_, v) in self.nodes.iter_mut() {
            v.sort_by(|a, b| b.distance.cmp(&a.distance));
        }
    }

    fn route_from(&mut self, start: &str) -> usize {
        let mut visited: Vec<String> = Vec::new();
        visited.push(start.to_string());
        let mut current = start;
        let mut distance = 0;
        while visited.len() != self.nodes.len() {
            let item = self.nodes.get(&current);
            let Some(edges) = item else {
                break;
            };
            for edge in edges {
                if visited.contains(&edge.node.to_string()) {
                    continue;
                }
                visited.push(current.to_string());
                current = edge.node;
                distance += edge.distance;
                break;
            }
        }
        return distance;
    }

    fn shortest_route(&mut self) -> usize {
        self.sort_min();
        let mut distance = usize::MAX;
        let starts: Vec<String> = self.nodes.keys().map(|x| x.to_string()).collect();
        for k in starts {
            let d = self.route_from(&k);
            if d < distance {
                distance = d;
            }
        }
        return distance;
    }

    fn longest_route(&mut self) -> usize {
        self.sort_max();
        let mut distance = 0;
        let starts: Vec<String> = self.nodes.keys().map(|x| x.to_string()).collect();
        for k in starts {
            let d = self.route_from(&k);
            if d > distance {
                distance = d;
            }
        }
        return distance;
    }
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let mut graph = Graph::new();
        for line in input.trim().lines() {
            graph.insert(line);
        }
        let d = graph.shortest_route();
        return format!("{}", d);
    }
    fn part2(&self, input: &str) -> String {
        let mut graph = Graph::new();
        for line in input.trim().lines() {
            graph.insert(line);
        }
        let d = graph.longest_route();
        return format!("{}", d);
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
        let line = r#"
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141"#;
        assert_eq!(solver.part1(line), "605");
    }

    #[test]
    fn test_part2() {
        let s = Solution;
        let solver: &dyn Solver = &s;
        let line = r#"
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141"#;
        assert_eq!(solver.part2(line), "982");
    }
}
