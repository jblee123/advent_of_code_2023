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
    let a = -1 as f64;
    let b = race.time as f64;
    let c = -((race.dist + 1) as f64);

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

pub fn get_num_of_ways_to_win_single_race(s: &str) -> u32 {
    let new_input = update_race_input(s);

    let race = parse_input_single(&new_input);

    get_num_wins(&race)
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
