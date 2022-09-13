use fancy_regex::Regex;
use lazy_static::lazy_static;

fn nice_string(s: &str) -> bool {
    lazy_static! {
        static ref R1: Regex = Regex::new(r"([aeiou].*){3}").unwrap();
        static ref R2: Regex = Regex::new(r"(.)\1").unwrap();
        static ref R3: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    }
    return R1.is_match(s).unwrap() && R2.is_match(s).unwrap() && !R3.is_match(s).unwrap();
}

fn nice_string2(s: &str) -> bool {
    lazy_static! {
        static ref R4: Regex = Regex::new(r"(..).*\1").unwrap();
        static ref R5: Regex = Regex::new(r"(.).\1").unwrap();
    }
    return R4.is_match(s).unwrap() && R5.is_match(s).unwrap();
}

fn main() {
    let data = include_str!("../input.txt").trim();

    let count = data.lines().filter(|l| nice_string(l)).count();
    println!("Part 1: {}", count);
    let count2 = data.lines().filter(|l| nice_string2(l)).count();
    println!("Part 2: {}", count2);
}

#[cfg(test)]
mod tests {
    use crate::nice_string;
    use crate::nice_string2;

    #[test]
    fn part_one_example_one() {
        assert_eq!(nice_string("ugknbfddgicrmopn"), true);
    }

    #[test]
    fn part_one_example_two() {
        assert_eq!(nice_string("aaa"), true);
    }

    #[test]
    fn part_one_example_three() {
        assert_eq!(nice_string("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn part_one_example_four() {
        assert_eq!(nice_string("haegwjzuvuyypxyu"), false);
    }

    #[test]
    fn part_one_example_five() {
        assert_eq!(nice_string("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn part_two_example_one() {
        assert_eq!(nice_string2("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn part_two_example_two() {
        assert_eq!(nice_string2("xxyxx"), true);
    }

    #[test]
    fn part_two_example_three() {
        assert_eq!(nice_string2("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn part_two_example_four() {
        assert_eq!(nice_string2("ieodomkazucvgmuy"), false);
    }
}
