struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn next(&mut self, direction: &char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => panic!("Unknown direction {direction}!"),
        }
    }

    fn to_string(&self) -> String {
        return format!("({},{})", self.x, self.y);
    }
}

struct Tracker {
    santa: Point,
    robo: Point,
    moves: usize,
    visited: Vec<String>,
}

impl Tracker {
    fn new() -> Tracker {
        return Tracker {
            santa: Point { x: 0, y: 0 },
            robo: Point { x: 0, y: 0 },
            moves: 0,
            visited: vec!["(0,0)".to_string()],
        };
    }

    fn next(&mut self, direction: &char) {
        let loc: String;
        if self.moves % 2 == 0 {
            self.santa.next(direction);
            loc = self.santa.to_string();
        } else {
            self.robo.next(direction);
            loc = self.robo.to_string();
        }
        self.moves += 1;
        if !self.visited.contains(&loc) {
            self.visited.push(loc);
        }
    }

    fn visited_houses(&self) -> usize {
        return self.visited.len();
    }
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut current_location = Point { x: 0, y: 0 };
    let mut visited = vec![current_location.to_string()];

    for character in data.chars() {
        current_location.next(&character);
        let at = current_location.to_string();
        if !visited.contains(&at) {
            visited.push(at);
        }
    }

    println!("Part 1: {}", visited.len());

    let mut t = Tracker::new();

    for character in data.chars() {
        t.next(&character);
    }

    println!("Part 2: {}", t.visited_houses());
}

#[cfg(test)]
mod tests {
    use crate::Tracker;

    #[test]
    fn pt_two_example_one() {
        let mut t = Tracker::new();
        for character in "^v".chars() {
            t.next(&character);
        }
        assert_eq!(t.visited_houses(), 3);
    }

    #[test]
    fn pt_two_example_two() {
        let mut t = Tracker::new();
        for character in "^>v<".chars() {
            t.next(&character);
        }
        assert_eq!(t.visited_houses(), 3);
    }

    #[test]
    fn pt_two_example_three() {
        let mut t = Tracker::new();
        for character in "^v^v^v^v^v".chars() {
            t.next(&character);
        }
        assert_eq!(t.visited_houses(), 11);
    }
}
