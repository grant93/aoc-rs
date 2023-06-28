use std::collections::{HashMap, HashSet};
use rstest::rstest;

fn apply_rules(input: String, part_two: bool) -> u8 {
    let mut chars: Vec<char> = input.chars().collect();
    let mut repeat = false;
    for i in 1..6 {
        if chars[i] < chars[i-1] { 
            return 0;
        } else if !repeat && chars[i] == chars[i-1] {
            if part_two && ((i != 5 && chars[i] == chars[i+1]) || (i != 1 && chars[i] == chars[i-2])) {
                continue;
            }
            repeat = true;
        }
    }
    if !repeat {
        return 0;
    }
    return 1;
}

fn run(input: String, part_two: bool) -> u32 {
    let (a, b) = input.split_at(6);
    let min = a.parse::<u32>().unwrap();
    let max = b[1..].parse::<u32>().unwrap();
    println!("{} {}", min, max);
    let mut count: u32 = 0;
    for i in min..=max {
        count += u32::from(apply_rules(i.to_string(), part_two));
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("137683-596253", 1864)]

    fn partone_test(#[case] input: String, #[case] expected: u32) {
        assert_eq!(expected, run(input, false))
    }
    #[rstest]
    #[case("137683-596253", 1258)]

    fn parttwo_test(#[case] input: String, #[case] expected: u32) {
        assert_eq!(expected, run(input, true))
    }

}
