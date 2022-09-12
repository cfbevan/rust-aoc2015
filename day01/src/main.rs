fn main() {
    let data = include_str!("../input.txt").trim();

    let mut floor: i32 = 0;
    let mut basement: usize = 0;

    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                panic!("unexpected character {c}")
            }
        }
        if basement == 0 && floor < 0 {
            basement = i + 1;
        }
    }
    println!("Part 1: {}", floor);

    println!("Part 2: {}", basement);
}
