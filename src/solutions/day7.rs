use std::collections::{HashMap, VecDeque};

use crate::solutions;
pub struct Solution;

#[derive(Debug)]
enum Wire<'a> {
    Input(u16),
    Name(&'a str),
}

fn parse_wire(token: &str) -> Wire {
    match token.parse() {
        Ok(v) => Wire::Input(v),
        Err(_) => Wire::Name(token),
    }
}

#[derive(Debug)]
enum Gate<'a> {
    Pipe(Wire<'a>, &'a str),
    And(Wire<'a>, Wire<'a>, &'a str),
    Or(Wire<'a>, Wire<'a>, &'a str),
    LShift(Wire<'a>, Wire<'a>, &'a str),
    RShift(Wire<'a>, Wire<'a>, &'a str),
    Not(Wire<'a>, &'a str),
}

fn parse_gate(line: &str) -> Gate {
    let mut tokens = line.trim().split("->");
    let left = tokens.next().unwrap().trim();
    let output = tokens.next().unwrap().trim();
    let tokens: Vec<&str> = left.split(' ').collect();
    if tokens.len() == 1 {
        let wire = parse_wire(tokens[0]);
        return Gate::Pipe(wire, output);
    };
    if tokens.len() == 2 {
        let wire = parse_wire(tokens[1]);
        return Gate::Not(wire, output);
    }
    if tokens.len() == 3 {
        let wire_a = parse_wire(tokens[0]);
        let gate = tokens[1];
        let wire_b = parse_wire(tokens[2]);
        match gate {
            "AND" => return Gate::And(wire_a, wire_b, output),
            "OR" => return Gate::Or(wire_a, wire_b, output),
            "LSHIFT" => return Gate::LShift(wire_a, wire_b, output),
            "RSHIFT" => return Gate::RShift(wire_a, wire_b, output),
            _ => unimplemented!(),
        }
    }
    unimplemented!();
}

struct ConnectionMap<'a> {
    map: HashMap<String, Gate<'a>>,
}

impl<'a> ConnectionMap<'a> {
    pub fn from_text(input: &'a str) -> Self {
        let mut map = HashMap::new();
        for line in input.lines() {
            let gate = parse_gate(line);
            let key = match gate {
                Gate::Pipe(_, x) => x.to_string(),
                Gate::And(_, _, x) => x.to_string(),
                Gate::Or(_, _, x) => x.to_string(),
                Gate::LShift(_, _, x) => x.to_string(),
                Gate::RShift(_, _, x) => x.to_string(),
                Gate::Not(_, x) => x.to_string(),
            };
            map.insert(key, gate);
        }
        return Self { map };
    }

    fn _wire_value(&self, wire: &'a Wire, table: &HashMap<&'a str, u16>) -> Result<u16, &'a str> {
        match wire {
            Wire::Input(v) => Ok(*v),
            Wire::Name(x) => {
                let Some(v) = table.get(x) else {
                    return Err(x);
                };
                Ok(*v)
            }
        }
    }

    pub fn signal_strength(&self, wire: &str) -> u16 {
        let mut queue: VecDeque<&str> = VecDeque::new();
        let mut table: HashMap<&str, u16> = HashMap::new();
        queue.push_back(wire);
        loop {
            let Some(current) = queue.pop_front() else {
                println!("err: queue is empty");
                return 0;
            };
            if let Some(&c_val) = table.get(current) {
                if queue.is_empty() {
                    return c_val;
                } else {
                    continue;
                }
            }
            let Some(c_gate) = self.map.get(current) else {
                println!("err: wire '{}'not found", current);
                return 0;
            };
            match c_gate {
                Gate::Pipe(x, out) => match self._wire_value(x, &table) {
                    Ok(x_val) => {
                        table.insert(out, x_val);
                        queue.push_front(out);
                    }
                    Err(x) => {
                        queue.push_front(out);
                        queue.push_front(&x);
                    }
                },
                Gate::Not(x, out) => match self._wire_value(x, &table) {
                    Ok(x_val) => {
                        table.insert(out, !x_val);
                        queue.push_front(out);
                    }
                    Err(x) => {
                        queue.push_front(out);
                        queue.push_front(&x);
                    }
                },
                Gate::And(x, y, out) => match self._wire_value(x, &table) {
                    Ok(x_val) => match self._wire_value(y, &table) {
                        Ok(y_val) => {
                            table.insert(out, x_val & y_val);
                            queue.push_front(out);
                        }
                        Err(y) => {
                            queue.push_front(out);
                            queue.push_front(&y);
                        }
                    },
                    Err(x) => {
                        queue.push_front(out);
                        queue.push_front(&x);
                    }
                },
                Gate::Or(x, y, out) => match self._wire_value(x, &table) {
                    Ok(x_val) => match self._wire_value(y, &table) {
                        Ok(y_val) => {
                            table.insert(out, x_val | y_val);
                            queue.push_front(out);
                        }
                        Err(y) => {
                            queue.push_front(out);
                            queue.push_front(&y);
                        }
                    },
                    Err(x) => {
                        queue.push_front(out);
                        queue.push_front(&x);
                    }
                },
                Gate::LShift(x, y, out) => match self._wire_value(x, &table) {
                    Ok(x_val) => match self._wire_value(y, &table) {
                        Ok(y_val) => {
                            table.insert(out, x_val << y_val);
                            queue.push_front(out);
                        }
                        Err(y) => {
                            queue.push_front(out);
                            queue.push_front(&y);
                        }
                    },
                    Err(x) => {
                        queue.push_front(out);
                        queue.push_front(&x);
                    }
                },
                Gate::RShift(x, y, out) => match self._wire_value(x, &table) {
                    Ok(x_val) => match self._wire_value(y, &table) {
                        Ok(y_val) => {
                            table.insert(out, x_val >> y_val);
                            queue.push_front(out);
                        }
                        Err(y) => {
                            queue.push_front(out);
                            queue.push_front(&y);
                        }
                    },
                    Err(x) => {
                        queue.push_front(out);
                        queue.push_front(&x);
                    }
                },
            };
            //println!("\tqueue: {:?}", queue); //, {:?}, {:?}")
            //println!("\ttable: {:?}", table); //, {:?}, {:?}")
        }
    }

    pub fn update_wire(&mut self, wire: &'a str, value: u16) {
        self.map
            .entry(wire.to_string())
            .and_modify(|x| *x = Gate::Pipe(Wire::Input(value), wire));
    }
}

impl solutions::Solver for Solution {
    fn part1(&self, input: &str) -> String {
        let map = ConnectionMap::from_text(input);

        return format!("{}", map.signal_strength("a"));
    }
    fn part2(&self, input: &str) -> String {
        let mut map = ConnectionMap::from_text(input);
        let signal_a = map.signal_strength("a");
        map.update_wire("b", signal_a);
        return format!("{}", map.signal_strength("a"));
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_connections() {
        let map = ConnectionMap::from_text(
            "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
i -> j",
        );

        assert_eq!(map.signal_strength("d"), 72);
        assert_eq!(map.signal_strength("e"), 507);
        assert_eq!(map.signal_strength("f"), 492);
        assert_eq!(map.signal_strength("g"), 114);
        assert_eq!(map.signal_strength("h"), 65412);
        assert_eq!(map.signal_strength("i"), 65079);
        assert_eq!(map.signal_strength("j"), 65079);
    }
}
