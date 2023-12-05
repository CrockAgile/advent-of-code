//! Something is wrong with global snow production, and you've been selected to take a look.
//! The Elves have even given you a map; on it, they've used stars to mark the top fifty locations
//! that are likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow operations,
//! you need to check all fifty stars by December 25th.
//! Collect stars by solving puzzles.
//! Two puzzles will be made available on each day in the Advent calendar;
//! the second puzzle is unlocked when you complete the first.
//! Each puzzle grants one star.
//!
//! Good luck! You try to ask why they can't just use a weather machine ("not powerful enough")
//! and where they're even sending you ("the sky") and why your map looks mostly blank
//! ("you sure ask a lot of questions") and hang on did you just say the sky
//! ("of course, where do you think snow comes from") when you realize that the Elves
//! are already loading you into a trebuchet ("please hold still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their calibration document
//! (your puzzle input) has been amended by a very young Elf who was apparently just excited
//! to show off her art skills. Consequently, the Elves are having trouble reading the values
//! on the document. The newly-improved calibration document consists of lines of text;
//! each line originally contained a specific calibration value that the Elves now need to recover.
//! On each line, the calibration value can be found by combining the first digit and the last digit
//! (in that order) to form a single two-digit number.
fn main() {
    let input = include_str!("./input.txt");
    let result = calibrate(input.split_ascii_whitespace());
    println!("result = {result}");
}

fn calibrate<'a>(s: impl Iterator<Item = &'a str>) -> usize {
    #[inline]
    fn parse<const IS_REV: bool>(line: &[u8]) -> Option<u8> {
        const PARSE_TABLE: &[(u8, &[u8])] = &[
            (1, "one".as_bytes()),
            (2, "two".as_bytes()),
            (3, "three".as_bytes()),
            (4, "four".as_bytes()),
            (5, "five".as_bytes()),
            (6, "six".as_bytes()),
            (7, "seven".as_bytes()),
            (8, "eight".as_bytes()),
            (9, "nine".as_bytes()),
        ];

        for (value, s) in PARSE_TABLE {
            let is_match = if IS_REV {
                line.starts_with(s)
            } else {
                line.ends_with(s)
            };
            if is_match {
                return Some(*value);
            }
        }

        let first = if IS_REV { line.first() } else { line.last() };
        if let Some(&value) = first {
            if value.is_ascii_digit() {
                return Some(value - b'0');
            }
        }

        None
    }

    let mut result = 0usize;

    for line in s {
        dbg!(line);
        let bytes = line.as_bytes();
        let len = bytes.len();
        let first = (1..=len)
            .map(|i| &bytes[..i])
            .find_map(parse::<false>)
            .unwrap();

        let last = (1..len)
            .map(|i| &bytes[len - i..])
            .find_map(parse::<true>)
            .unwrap_or(first);

        let answer = (first * 10 + last) as usize;
        result += answer;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        assert_eq!(142, calibrate(input.into_iter()));
    }

    #[test]
    fn part_two() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        assert_eq!(281, calibrate(input.into_iter()));
    }
}
