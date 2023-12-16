fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ')
        .map(|part| part.parse().unwrap())
        .collect::<Vec<i64>>()
}

fn get_line_diffs(vals: &Vec<i64>) -> Vec<i64> {
    let mut new_vals = Vec::<i64>::default();
    new_vals.reserve(vals.len() - 1);

    let mut idx = 1 as usize;
    while idx < vals.len() {
        new_vals.push(vals[idx] - vals[idx - 1]);
        idx += 1;
    }

    new_vals
}

fn all_zeros(vals: &Vec<i64>) -> bool {
    vals.iter().all(|val| *val == 0)
}

fn extrapolate_line(vals: &Vec<i64>) -> i64 {
    let mut value_sets = vec![vals.clone()];

    loop {
        value_sets.push(get_line_diffs(value_sets.last().unwrap()));

        if all_zeros(value_sets.last().unwrap()) {
            break;
        }
    }

    value_sets
        .iter()
        .map(|value_set| value_set.last().unwrap())
        .sum()
}

fn extrapolate_line_back(vals: &Vec<i64>) -> i64 {
    let mut value_sets = vec![vals.clone()];

    loop {
        value_sets.push(get_line_diffs(value_sets.last().unwrap()));

        if all_zeros(value_sets.last().unwrap()) {
            break;
        }
    }

    let first_vals = value_sets
        .iter()
        .map(|value_set| *value_set.first().unwrap())
        .collect::<Vec<i64>>();

    let mut result = 0;
    let mut multiplier = 1;
    first_vals.iter().for_each(|val| {
        result += *val * multiplier;
        multiplier *= -1;
    });

    result
}

pub fn sum_extrapolated_values(s: &str) -> i64 {
    s.lines()
        .map(|line| parse_line(line))
        .map(|vals| extrapolate_line(&vals))
        .sum()
}

pub fn sum_extrapolated_back_values(s: &str) -> i64 {
    s.lines()
        .map(|line| parse_line(line))
        .map(|vals| extrapolate_line_back(&vals))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "0 3 6 9 12 15\n",
        "1 3 6 10 15 21\n",
        "10 13 16 21 30 45\n",
    );

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("0 3 6 9 12 15"), vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_get_line_diffs() {
        assert_eq!(
            get_line_diffs(&vec![0, 3, 6, 9, 12, 15]),
            vec![3, 3, 3, 3, 3]
        );
    }

    #[test]
    fn test_extrapolate_line() {
        assert_eq!(extrapolate_line(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate_line(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate_line(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_extrapolate_line_back() {
        assert_eq!(extrapolate_line_back(&vec![10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn test_sum_extrapolated_values() {
        assert_eq!(sum_extrapolated_values(SAMPLE_INPUT), 114);
    }

    #[test]
    fn test_sum_extrapolated_back_values() {
        assert_eq!(sum_extrapolated_back_values(SAMPLE_INPUT), 2);
    }
}
