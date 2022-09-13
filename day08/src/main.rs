use escape_string;
use unescaper;

fn code_count(s: &str) -> usize {
    return s.len();
}

fn mem_count(s: &str) -> usize {
    let decoded = unescaper::unescape(s).unwrap();
    return decoded.chars().count() - 2; // subtract 2 for the quotes
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let mut total_code_count = 0;
    let mut total_mem_count = 0;
    for line in data.lines() {
        total_code_count += code_count(line);
        total_mem_count += mem_count(line);
    }
    println!("Part 1: {}", total_code_count - total_mem_count);

    println!("Part 2: {}", "");
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::code_count;
    use crate::mem_count;
    use escape_string;

    #[test]
    fn part_one_example_one() {
        let test = r#""""#;
        assert_eq!(code_count(test), 2, "code count");
        assert_eq!(mem_count(test), 0, "mem count");
    }

    #[test]
    fn part_one_example_two() {
        let test = r#""abc""#;
        assert_eq!(code_count(test), 5, "code count");
        assert_eq!(mem_count(test), 3, "mem count");
    }

    #[test]
    fn part_one_example_three() {
        let test = r#""aaa\"aaa""#;
        assert_eq!(code_count(test), 10, "code count");
        assert_eq!(mem_count(test), 7, "mem count");
    }

    #[test]
    fn part_one_example_four() {
        let test = r#""\x27""#;
        assert_eq!(code_count(test), 6, "code count");
        assert_eq!(mem_count(test), 1, "mem count");
    }

    // part two

    #[test]
    fn part_two_example_one() {
        let test = r#""""#;
        let encoded = &escape_string::escape(&test);
        dbg!(encoded);
        assert_eq!(code_count(encoded), 6, "code count");
        assert_eq!(mem_count(encoded), 0, "mem count");
    }

    #[test]
    fn part_two_example_two() {
        let test = r#""abc""#;
        let encoded = &escape_string::escape(test);
        dbg!(encoded);
        assert_eq!(code_count(encoded), 9, "code count");
        assert_eq!(mem_count(encoded), 3, "mem count");
    }

    #[test]
    fn part_two_example_three() {
        let test = r#""aaa\"aaa""#;
        let encoded = &escape_string::escape(test);
        dbg!(encoded);
        assert_eq!(code_count(encoded), 16, "code count");
        assert_eq!(mem_count(encoded), 7, "mem count");
    }

    #[test]
    fn part_two_example_four() {
        let test = r#""\x27""#;
        let encoded = &escape_string::escape(test);
        dbg!(encoded);
        assert_eq!(code_count(encoded), 11, "code count");
        assert_eq!(mem_count(encoded), 1, "mem count");
    }
}
