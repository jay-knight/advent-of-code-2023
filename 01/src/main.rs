use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn get_calibration_value(string: &str) -> u32 {
    let digit_map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut first_digit = 0;
    let mut first_digit_index = string.len();
    let mut last_digit = 0;
    let mut last_digit_index = 0;
    for (digit_str, digit) in digit_map.into_iter() {
        if let Some(first_match) = string.find(digit_str) {
            if first_match <= first_digit_index {
                first_digit = digit;
                first_digit_index = first_match;
            }
        }
        if let Some(last_match) = string.rfind(digit_str) {
            if last_match >= last_digit_index {
                last_digit = digit;
                last_digit_index = last_match;
            }
        }
    }
    first_digit * 10 + last_digit
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(line_str) = line {
                let calibration_value = get_calibration_value(&line_str);
                //println!("{} -> {}", line_str, calibration_value);
                sum += calibration_value;
            }
        }
        println!("Sum of Calibration values: {}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::get_calibration_value;

    #[test]
    fn test_calibration_values() {
        // Part 1:
        assert_eq!(12, get_calibration_value("1abc2"));
        assert_eq!(38, get_calibration_value("pqr3stu8vwx"));
        assert_eq!(15, get_calibration_value("a1b2c3d4e5f"));
        assert_eq!(77, get_calibration_value("treb7uchet"));
        // Part 2:
        assert_eq!(29, get_calibration_value("two1nine"));
        assert_eq!(83, get_calibration_value("eightwothree"));
        assert_eq!(13, get_calibration_value("abcone2threexyz"));
        assert_eq!(24, get_calibration_value("xtwone3four"));
        assert_eq!(42, get_calibration_value("4nineeightseven2"));
        assert_eq!(14, get_calibration_value("zoneight234"));
        assert_eq!(76, get_calibration_value("7pqrstsixteen"));
        // Problematic:
        assert_eq!(44, get_calibration_value("4l"));
    }
}
