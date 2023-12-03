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
    fn digit_val(b: u8) -> u8 {
        b - b'0'
    }

    let mut result = 0usize;

    for line in s {
        let bytes = line.as_bytes();
        let first = bytes.iter().find(|b| b.is_ascii_digit()).unwrap();
        let last = bytes.iter().rev().find(|b| b.is_ascii_digit()).unwrap();
        let first = digit_val(*first);
        let last = digit_val(*last);
        result += (first * 10 + last) as usize;
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
}
