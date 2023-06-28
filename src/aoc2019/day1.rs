use crate::utils::file;
use rstest::rstest;

fn partone(input: &mut Vec<i64>) -> i64 {
    input.iter().map(|x| (x/3) - 2).sum()
}

fn recurse(input: i64) -> i64 {
    let a = (input / 3) - 2;
    if a <= 0 {
        return 0
    }
    a + recurse(a)
}

fn parttwo(input: &mut Vec<i64>) -> i64 {
    input.iter().map(|x| recurse(*x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("src/aoc2019/input/day1.txt", 3455717)]
    fn partone_test(#[case] input: &str, #[case] expected: i64) {
        let mut input = file::read_lines_to_i64s(input);
        assert_eq!(expected, partone(&mut input))
    }
    #[rstest]
    #[case("src/aoc2019/input/day1.txt", 5180690)]
    fn parttwo_test(#[case] input: &str, #[case] expected: i64) {
        let mut input = file::read_lines_to_i64s(input);
        assert_eq!(expected, parttwo(&mut input))
    }

}
