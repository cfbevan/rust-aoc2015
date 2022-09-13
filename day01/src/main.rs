struct Elevator {
    floor: isize,
    basement: usize,
}

impl Elevator {
    fn new() -> Elevator {
        return Elevator {
            floor: 0,
            basement: 0,
        };
    }

    fn run(&mut self, data: &str) {
        for (i, c) in data.chars().enumerate() {
            match c {
                '(' => self.floor += 1,
                ')' => self.floor -= 1,
                _ => {
                    panic!("unexpected character {c}")
                }
            }
            if self.basement == 0 && self.floor < 0 {
                self.basement = i + 1;
            }
        }
    }
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut e: Elevator = Elevator::new();
    e.run(data);

    println!("Part 1: {}", e.floor);

    println!("Part 2: {}", e.basement);
}

#[cfg(test)]
mod tests {
    use crate::Elevator;

    #[test]
    fn part_one_example_one_a() {
        let mut e: Elevator = Elevator::new();
        e.run("(())");
        assert_eq!(e.floor, 0);
    }

    #[test]
    fn part_one_example_one_b() {
        let mut e: Elevator = Elevator::new();
        e.run("()()");
        assert_eq!(e.floor, 0);
    }

    #[test]
    fn part_one_example_two_a() {
        let mut e: Elevator = Elevator::new();
        e.run("(((");
        assert_eq!(e.floor, 3);
    }

    #[test]
    fn part_one_example_two_b() {
        let mut e: Elevator = Elevator::new();
        e.run("(()(()(");
        assert_eq!(e.floor, 3);
    }

    #[test]
    fn part_one_example_three() {
        let mut e: Elevator = Elevator::new();
        e.run("))(((((");
        assert_eq!(e.floor, 3);
    }

    #[test]
    fn part_one_example_four_a() {
        let mut e: Elevator = Elevator::new();
        e.run("())");
        assert_eq!(e.floor, -1);
    }

    #[test]
    fn part_one_example_four_b() {
        let mut e: Elevator = Elevator::new();
        e.run("))(");
        assert_eq!(e.floor, -1);
    }

    #[test]
    fn part_one_example_five_a() {
        let mut e: Elevator = Elevator::new();
        e.run(")))");
        assert_eq!(e.floor, -3);
    }

    #[test]
    fn part_one_example_five_b() {
        let mut e: Elevator = Elevator::new();
        e.run(")())())");
        assert_eq!(e.floor, -3);
    }

    #[test]
    fn part_two_example_one() {
        let mut e: Elevator = Elevator::new();
        e.run(")");
        assert_eq!(e.basement, 1);
    }

    #[test]
    fn part_two_example_two() {
        let mut e: Elevator = Elevator::new();
        e.run("()())");
        assert_eq!(e.basement, 5);
    }
}
