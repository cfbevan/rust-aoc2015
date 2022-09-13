use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Operator {
    ASSIGN,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

#[derive(Clone, Debug)]
struct Gate {
    arg1: String,
    arg2: String,
    op: Operator,
}

impl Gate {
    fn from(s: &str) -> Gate {
        let parser1: Regex =
            Regex::new(r"(?P<arg1>.*) (?P<op>AND|OR|LSHIFT|RSHIFT) (?P<arg2>.*)").unwrap();
        let parser2: Regex = Regex::new(r"(?P<op>NOT) (?P<arg1>.*)").unwrap();
        let parser3: Regex = Regex::new(r"(?P<arg1>.*)").unwrap();
        let captures;
        if parser1.is_match(s) {
            captures = parser1.captures(s).unwrap();
        } else if parser2.is_match(s) {
            captures = parser2.captures(s).unwrap();
        } else if parser3.is_match(s) {
            captures = parser3.captures(s).unwrap();
        } else {
            panic!("Gate failed to parse: {s}");
        }
        let op_str = captures.name("op").map_or("ASSIGN", |o| o.as_str());
        let arg1_str = captures.name("arg1").map_or("", |o| o.as_str()).to_string();
        let arg2_str = captures.name("arg2").map_or("", |o| o.as_str()).to_string();
        let op = match op_str {
            "ASSIGN" => Operator::ASSIGN,
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "LSHIFT" => Operator::LSHIFT,
            "RSHIFT" => Operator::RSHIFT,
            "NOT" => Operator::NOT,
            _ => panic!("Not an operator: {op_str}"),
        };
        return Gate {
            arg1: arg1_str,
            arg2: arg2_str,
            op,
        };
    }

    fn eval(&self, arg1: &u16, arg2: &u16) -> u16 {
        match self.op {
            Operator::ASSIGN => arg1.clone(),
            Operator::NOT => !arg1,
            Operator::AND => arg1 & arg2,
            Operator::OR => arg1 | arg2,
            Operator::LSHIFT => arg1 << arg2,
            Operator::RSHIFT => arg1 >> arg2,
        }
    }
}

struct Command {
    gate: Gate,
    destination: String,
}

impl Command {
    fn from(s: &str) -> Command {
        let parser: Regex = Regex::new(r"(?P<gate>.*) -> (?P<destination>[a-z]*)").unwrap();
        let captures = parser.captures(s).ok_or("no matches").unwrap();
        let g: Gate = Gate::from(captures.name("gate").map_or("", |c| c.as_str()));
        let dest = captures
            .name("destination")
            .map_or("", |c| c.as_str())
            .to_string();
        return Command {
            gate: g,
            destination: dest,
        };
    }
}

struct Circuit {
    commands: HashMap<String, Gate>,
    outputs: HashMap<String, u16>,
}

impl Circuit {
    fn new(data: &str) -> Circuit {
        let mut c = Circuit {
            commands: HashMap::new(),
            outputs: HashMap::new(),
        };
        for line in data.lines() {
            let cmd = Command::from(line);
            c.commands.insert(cmd.destination, cmd.gate);
        }
        return c;
    }

    fn get_value(&mut self, arg: &str) -> u16 {
        // catch issues where gate does not have an arg2
        if arg == "" {
            return 0;
        }
        // check if number
        if arg.chars().all(|c| c.is_ascii_digit()) {
            return arg.parse::<u16>().unwrap();
        }
        // check if we already have value
        if self.outputs.contains_key(arg) {
            return *self.outputs.get(arg).unwrap();
        }
        // calculate value
        let gate: Gate = self.commands.get(arg).unwrap().clone();
        let arg1 = self.get_value(&gate.arg1);
        let arg2 = self.get_value(&gate.arg2);
        let result = gate.eval(&arg1, &arg2);
        self.outputs.insert(arg.to_string(), result);
        return result;
    }
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut c = Circuit::new(data);

    println!("Part 1: {}", c.get_value("a"));

    let mut c2 = Circuit::new(data);
    let cmd = Command::from("46065 -> b");
    c2.commands.insert(cmd.destination, cmd.gate);

    println!("Part 2: {}", c2.get_value("a"));
}

#[cfg(test)]
mod tests {
    use crate::Circuit;

    #[test]
    fn part_one_example_one() {
        let commands = [
            "123 -> x\n",
            "456 -> y\n",
            "x AND y -> d\n",
            "x OR y -> e\n",
            "x LSHIFT 2 -> f\n",
            "y RSHIFT 2 -> g\n",
            "NOT x -> h\n",
            "NOT y -> i\n",
        ]
        .concat();
        let mut c: Circuit = Circuit::new(&commands);
        assert_eq!(c.get_value("d"), 72u16);
        assert_eq!(c.get_value("e"), 507u16);
        assert_eq!(c.get_value("f"), 492u16);
        assert_eq!(c.get_value("g"), 114u16);
        assert_eq!(c.get_value("h"), 65412u16);
        assert_eq!(c.get_value("i"), 65079u16);
        assert_eq!(c.get_value("x"), 123u16);
        assert_eq!(c.get_value("y"), 456u16);
    }
}
