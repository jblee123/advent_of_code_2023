pub fn get_calibration_value(s: &str) -> u32 {
    let c1 = s.chars().find(|c| c.is_digit(10)).unwrap();
    let c2 = s.chars().rfind(|c| c.is_digit(10)).unwrap();
    let d1 = c1.to_digit(10).unwrap();
    let d2 = c2.to_digit(10).unwrap();
    d1 * 10 + d2
}

pub fn get_calibration_value2(s: &str) -> u32 {
    let number_strs = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut pos1 = usize::MAX;
    let mut val1 = 0 as u32;
    for (idx, num_str) in number_strs.iter().enumerate() {
        if let Some(pos) = s.find(num_str) {
            if pos < pos1 {
                pos1 = pos;
                val1 = numbers[idx] as u32;
            }
        }
    }

    let mut pos2 = usize::MIN;
    let mut val2 = 0 as u32;
    for (idx, num_str) in number_strs.iter().enumerate() {
        if let Some(pos) = s.rfind(num_str) {
            if pos >= pos2 {
                pos2 = pos;
                val2 = numbers[idx] as u32;
            }
        }
    }

    val1 * 10 + val2
}

pub fn sum_calibration_values(s: &str) -> u32 {
    s.lines().map(|line| get_calibration_value(line)).sum()
}

pub fn sum_calibration_values2(s: &str) -> u32 {
    s.lines().map(|line| get_calibration_value2(line)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_value() {
        assert_eq!(get_calibration_value("1abc2"), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn test_get_calibration_value2() {
        assert_eq!(get_calibration_value2("two1nine"), 29);
        assert_eq!(get_calibration_value2("eightwothree"), 83);
        assert_eq!(get_calibration_value2("abcone2threexyz"), 13);
        assert_eq!(get_calibration_value2("xtwone3four"), 24);
        assert_eq!(get_calibration_value2("4nineeightseven2"), 42);
        assert_eq!(get_calibration_value2("zoneight234"), 14);
        assert_eq!(get_calibration_value2("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_sum_calibration_values() {
        let input = concat!("1abc2\n", "pqr3stu8vwx\n", "a1b2c3d4e5f\n", "treb7uchet\n",);
        assert_eq!(sum_calibration_values(input), 142);
    }

    #[test]
    fn test_sum_calibration_values2() {
        let input = concat!(
            "two1nine\n",
            "eightwothree\n",
            "abcone2threexyz\n",
            "xtwone3four\n",
            "4nineeightseven2\n",
            "zoneight234\n",
            "7pqrstsixteen\n",
        );
        assert_eq!(sum_calibration_values2(input), 281);
    }
}
