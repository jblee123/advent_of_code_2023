#[derive(Debug, PartialEq, Copy, Clone)]
struct Race {
    time: u64,
    dist: u64,
}

fn parse_input(s: &str) -> Vec<Race> {
    let mut lines = s.lines();
    let time_line = lines.next().unwrap();
    let dist_line = lines.next().unwrap();

    let time_parts = time_line
        .split(' ')
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>();
    let dist_parts = dist_line
        .split(' ')
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>();

    std::iter::zip(&time_parts[1..], &dist_parts[1..])
        .map(|(time_str, dist_str)| Race {
            time: time_str.parse().unwrap(),
            dist: dist_str.parse().unwrap(),
        })
        .collect::<Vec<Race>>()
}

fn parse_input_single(s: &str) -> Race {
    let mut lines = s.lines();
    let time_line = lines.next().unwrap();
    let dist_line = lines.next().unwrap();

    let time = time_line
        .split(' ')
        .filter(|part| !part.is_empty())
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let dist = dist_line
        .split(' ')
        .filter(|part| !part.is_empty())
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    Race { time, dist }
}

fn get_num_wins(race: &Race) -> u32 {
    let a = -1 as f32;
    let b = race.time as f32;
    let c = -((race.dist + 1) as f32);

    let sqrt_part = ((b * b) - (4. * a * c)).sqrt();

    let ans1 = ((-b + sqrt_part) / (2. * a)).ceil() as u32;
    let ans2 = ((-b - sqrt_part) / (2. * a)).floor() as u32;

    ans2 - ans1 + 1
}

pub fn get_product_of_ways_to_win(s: &str) -> u32 {
    let races = parse_input(s);
    races.iter().map(|race| get_num_wins(race)).product()
}

fn update_race_line(line: &str) -> String {
    let digit_idx = line.find(|c: char| c.is_digit(10)).unwrap();
    let digits = line[digit_idx..]
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>();
    let start = line[0..digit_idx].to_owned();
    start + &digits
}

fn update_race_input(s: &str) -> String {
    let lines = s.lines();
    let mut new_input = String::default();
    for new_line in lines.map(|line| update_race_line(line) + "\n") {
        new_input.push_str(&new_line);
    }
    new_input
}

fn sqrt64_floor(n: u64) -> (u64, bool) {
    if n == 0 {
        return (0, true);
    }
    if n == 1 {
        return (1, true);
    }

    let mut min = 1 as u64;
    let mut max = 2 as u64;
    while (max * max) < n {
        min = max;
        max += max;
    }
    while (max - min) > 1 {
        let mid = (min + max) / 2;
        let mid_sq = mid * mid;
        if mid_sq <= n {
            min = mid;
        } else {
            max = mid;
        }
    }

    if (max * max) == n {
        (max, true)
    } else {
        (min, (min * min) == n)
    }
}

pub fn get_num_of_ways_to_win_single_race(s: &str) -> u64 {
    let new_input = update_race_input(s);

    let race = parse_input_single(&new_input);

    // Quadratic formula
    // T = time
    // D = dist + 1
    // x * Tx - D
    // -x^2 + Tx - D
    // [-T +/- sqrt(T^2 - 4(-1)(-D))] / 2(-1)
    // [-T +/- sqrt(T^2 - 4D)] / -2
    // [T +/- sqrt(T^2 - 4D)] / 2
    let quot = (race.time * race.time) - (4 * (race.dist + 1));
    let (floor_sqrt_quot, sqrt_exact) = sqrt64_floor(quot);

    let numerator1 = race.time - floor_sqrt_quot;
    let numerator2 = race.time + floor_sqrt_quot;
    let ans1_exact = sqrt_exact && ((numerator1 % 2) == 0);
    // let ans2_exact = sqrt_exact && ((numerator2 % 2) == 0);

    let mut ans1 = numerator1 / 2;
    if !ans1_exact {
        ans1 += 1;
    }
    let ans2 = numerator2 / 2;

    ans2 - ans1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "Time:      7  15   30\n", //
        "Distance:  9  40  200\n", //
    );

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE_INPUT),
            vec![
                Race { time: 7, dist: 9 },
                Race { time: 15, dist: 40 },
                Race {
                    time: 30,
                    dist: 200
                },
            ]
        );
    }

    #[test]
    fn test_get_num_wins() {
        assert_eq!(get_num_wins(&Race { time: 7, dist: 9 }), 4);
        assert_eq!(get_num_wins(&Race { time: 15, dist: 40 }), 8);
        assert_eq!(
            get_num_wins(&Race {
                time: 30,
                dist: 200
            }),
            9
        );
    }

    #[test]
    fn test_get_produce_of_ways_to_win() {
        assert_eq!(get_product_of_ways_to_win(SAMPLE_INPUT), 288);
    }

    #[test]
    fn test_update_race_line() {
        assert_eq!(
            update_race_line("Time:      7  15   30"),
            "Time:      71530"
        );
        assert_eq!(
            update_race_line("Distance:  9  40  200"),
            "Distance:  940200"
        );
    }

    #[test]
    fn test_sqrt64_floor() {
        assert_eq!(sqrt64_floor(0), (0, true));
        assert_eq!(sqrt64_floor(1), (1, true));
        assert_eq!(sqrt64_floor(2), (1, false));
        assert_eq!(sqrt64_floor(3), (1, false));
        assert_eq!(sqrt64_floor(4), (2, true));
        assert_eq!(sqrt64_floor(5), (2, false));
        assert_eq!(sqrt64_floor(6), (2, false));
        assert_eq!(sqrt64_floor(7), (2, false));
        assert_eq!(sqrt64_floor(8), (2, false));
        assert_eq!(sqrt64_floor(9), (3, true));
        assert_eq!(sqrt64_floor(15), (3, false));
        assert_eq!(sqrt64_floor(16), (4, true));
        assert_eq!(sqrt64_floor(17), (4, false));
        assert_eq!(sqrt64_floor(24), (4, false));
        assert_eq!(sqrt64_floor(25), (5, true));
        assert_eq!(sqrt64_floor(26), (5, false));
        assert_eq!(sqrt64_floor(2394973454433156), (48938466, true));
    }

    #[test]
    fn test_update_race_input() {
        assert_eq!(
            update_race_input(SAMPLE_INPUT),
            "Time:      71530\nDistance:  940200\n"
        );
    }

    #[test]
    fn test_get_num_of_ways_to_win_single_race() {
        assert_eq!(get_num_of_ways_to_win_single_race(SAMPLE_INPUT), 71503);
    }
}
