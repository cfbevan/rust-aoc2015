fn check_string(s: String, zeros: usize) -> bool {
    let digest = md5::compute(s.as_bytes());
    let starts_with: String = "0".repeat(zeros);
    return format!("{:x}", digest).starts_with(&starts_with);
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut counter: usize = 0;
    while !check_string(format!("{}{}", data, counter), 5) {
        counter += 1;
    }
    println!("Part 1: {}", counter);

    counter = 0;
    while !check_string(format!("{}{}", data, counter), 6) {
        counter += 1;
    }

    println!("Part 2: {}", counter);
}

#[cfg(test)]
mod tests {
    use crate::check_string;

    #[test]
    fn part_one_example_one() {
        assert_eq!(check_string("abcdef609043".to_string(), 5), true);
    }

    #[test]
    fn part_one_example_two() {
        assert_eq!(check_string("pqrstuv1048970".to_string(), 5), true);
    }
}
