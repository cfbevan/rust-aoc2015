use regex::Regex;

struct Point {
    r: usize,
    c: usize,
}

enum Instruction {
    On,
    Off,
    Toggle,
}

struct Cmd {
    instruction: Instruction,
    start: Point,
    end: Point,
}

impl Cmd {
    fn from(s: &str) -> Cmd {
        let parser = Regex::new(
            r"(?P<cmd>turn off|turn on|toggle) (?P<start>\d+,\d+) through (?P<end>\d+,\d+)",
        )
        .unwrap();
        let captures = parser.captures(s).ok_or("no matches").unwrap();
        let command = captures.name("cmd").map_or("", |c| c.as_str()).to_string();
        let cmd = match command.as_str() {
            "turn on" => Instruction::On,
            "turn off" => Instruction::Off,
            "toggle" => Instruction::Toggle,
            _ => panic!("Failed to parse instruction: {command} is not knonw"),
        };
        let start: Vec<usize> = captures
            .name("start")
            .map_or("", |s| s.as_str())
            .split(',')
            .map(|s| s.parse().expect("Failed to parse start: {s} not an int"))
            .collect();
        let end: Vec<usize> = captures
            .name("end")
            .map_or("", |e| e.as_str())
            .split(',')
            .map(|e| e.parse().expect("Failed to parse end: {e} not an int"))
            .collect();
        return Cmd {
            instruction: cmd,
            start: Point {
                r: start[0],
                c: start[1],
            },
            end: Point {
                r: end[0],
                c: end[1],
            },
        };
    }
}

struct Grid {
    lights: [[u8; 1000]; 1000],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            lights: [[0; 1000]; 1000],
        }
    }

    fn execute(&mut self, cmd: &Cmd) {
        for r in cmd.start.r..cmd.end.r + 1 {
            for c in cmd.start.c..cmd.end.c + 1 {
                match cmd.instruction {
                    Instruction::Off => self.lights[r][c] = 0,
                    Instruction::On => self.lights[r][c] = 1,
                    Instruction::Toggle => self.lights[r][c] = (self.lights[r][c] + 1) % 2,
                }
            }
        }
    }

    fn execute2(&mut self, cmd: &Cmd) {
        for r in cmd.start.r..cmd.end.r + 1 {
            for c in cmd.start.c..cmd.end.c + 1 {
                match cmd.instruction {
                    Instruction::Off => {
                        if self.lights[r][c] > 0 {
                            self.lights[r][c] -= 1
                        }
                    }
                    Instruction::On => self.lights[r][c] += 1,
                    Instruction::Toggle => self.lights[r][c] += 2,
                }
            }
        }
    }

    fn count_on(&self) -> usize {
        return self.lights.iter().fold(0, |count, row| {
            count
                + row
                    .iter()
                    .fold(0, |r_count, light| r_count + *light as usize) as usize
        });
    }
}
fn main() {
    let data = include_str!("../input.txt").trim();

    let mut g = Grid::new();
    for line in data.lines() {
        let c = Cmd::from(line);
        g.execute(&c);
    }
    println!("Part 1: {}", g.count_on());
    let mut g2 = Grid::new();
    for line in data.lines() {
        let c = Cmd::from(line);
        g2.execute2(&c);
    }
    println!("Part 2: {}", g2.count_on());
}

#[cfg(test)]
mod tests {
    use crate::Cmd;
    use crate::Grid;

    #[test]
    fn part_one_example_one() {
        let mut g = Grid::new();
        let c: Cmd = Cmd::from("turn on 0,0 through 999,999");
        g.execute(&c);
        assert_eq!(g.count_on(), 1000000);
    }

    #[test]
    fn part_one_example_two() {
        let mut g = Grid::new();
        let c: Cmd = Cmd::from("toggle 0,0 through 999,0");
        g.execute(&c);
        assert_eq!(g.count_on(), 1000);
    }

    #[test]
    fn part_two_example_one() {
        let mut g = Grid::new();
        let c: Cmd = Cmd::from("turn on 0,0 through 0,0");
        g.execute2(&c);
        assert_eq!(g.count_on(), 1);
    }

    #[test]
    fn part_two_example_two() {
        let mut g = Grid::new();
        let c: Cmd = Cmd::from("toggle 0,0 through 999,999");
        g.execute2(&c);
        assert_eq!(g.count_on(), 2000000);
    }
}
