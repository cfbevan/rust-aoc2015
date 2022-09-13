struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    fn new(data: &str) -> Box {
        // parse data string
        let mut dims: Vec<usize> = data
            .split('x')
            .map(|d| d.parse().expect("parse error"))
            .collect();
        dims.sort();
        return Box {
            l: dims[0],
            w: dims[1],
            h: dims[2],
        };
    }
    fn surface_area(&self) -> usize {
        // this works because we sorted the dimensions.
        // therefore l and w will be the smaller area
        // so we multiply that by 3 instead of 2
        return (3 * self.l * self.w) + (2 * self.l * self.h) + (2 * self.h * self.w);
    }
    fn ribbon_length(&self) -> usize {
        // perimter of smallest face plus volume
        // since we sort the dimensions, the smallest face
        // is always LxW
        return (2 * self.l) + (2 * self.w) + (self.l * self.w * self.h);
    }
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut total_area = 0;
    let mut total_ribbon = 0;

    for line in data.lines() {
        let b: Box = Box::new(line);
        total_area += b.surface_area();
        total_ribbon += b.ribbon_length();
    }

    println!("Part 1: {}", total_area);

    println!("Part 2: {}", total_ribbon);
}

#[cfg(test)]
mod tests {
    use crate::Box;

    #[test]
    fn part_one_example_one() {
        let b: Box = Box::new("2x3x4");
        assert_eq!(b.surface_area(), 58);
    }

    #[test]
    fn part_one_example_two() {
        let b: Box = Box::new("1x1x10");
        assert_eq!(b.surface_area(), 43);
    }

    #[test]
    fn part_two_example_one() {
        let b: Box = Box::new("2x3x4");
        assert_eq!(b.ribbon_length(), 34);
    }

    #[test]
    fn part_two_example_two() {
        let b: Box = Box::new("1x1x10");
        assert_eq!(b.ribbon_length(), 14);
    }
}
