#[derive(Debug, PartialEq)]
struct Handful {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

fn parse_handful(s: &str) -> Handful {
    let mut handful = Handful {
        num_red: 0,
        num_green: 0,
        num_blue: 0,
    };
    for cubeset_str in s.split(", ") {
        let parts = cubeset_str.split(' ').collect::<Vec<&str>>();
        let num: u32 = parts[0].parse().unwrap();
        match parts[1] {
            "red" => handful.num_red = num,
            "green" => handful.num_green = num,
            "blue" => handful.num_blue = num,
            _ => panic!("Bad color: {}", parts[1]),
        };
    }

    handful
}

fn parse_line(line: &str) -> Game {
    const ID_START_IDX: usize = 5;
    let colon_idx = line.find(':').unwrap();
    let id_str = &line[ID_START_IDX..colon_idx];
    let id = u32::from_str_radix(id_str, 10).unwrap();

    let handfuls_str = &line[(colon_idx + 2)..];
    let handfuls = handfuls_str
        .split("; ")
        .into_iter()
        .map(|handful_str| parse_handful(handful_str))
        .collect::<Vec<Handful>>();

    Game {
        id: id,
        handfuls: handfuls,
    }
}

fn get_max_handful_game(game: &Game) -> Game {
    let mut max_handful = Handful {
        num_red: 0,
        num_green: 0,
        num_blue: 0,
    };

    for handful in &game.handfuls {
        max_handful.num_red = max_handful.num_red.max(handful.num_red);
        max_handful.num_green = max_handful.num_green.max(handful.num_green);
        max_handful.num_blue = max_handful.num_blue.max(handful.num_blue);
    }

    Game {
        id: game.id,
        handfuls: vec![max_handful],
    }
}

pub fn get_sum_of_possible_game_ids(s: &str, red: u32, green: u32, blue: u32) -> u32 {
    s.lines()
        .map(|line| parse_line(line))
        .map(|game| get_max_handful_game(&game))
        .filter(|game| {
            (game.handfuls[0].num_red <= red)
                && (game.handfuls[0].num_green <= green)
                && (game.handfuls[0].num_blue <= blue)
        })
        .map(|game| game.id)
        .sum()
}

pub fn get_sum_of_power_of_min_possible_sets(s: &str) -> u32 {
    s.lines()
        .map(|line| parse_line(line))
        .map(|game| get_max_handful_game(&game))
        .map(|game| {
            game.handfuls[0].num_red * game.handfuls[0].num_green * game.handfuls[0].num_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_handful() {
        assert_eq!(
            parse_handful("3 blue, 4 red"),
            Handful {
                num_red: 4,
                num_green: 0,
                num_blue: 3
            }
        );
        assert_eq!(
            parse_handful("1 red, 2 green, 6 blue"),
            Handful {
                num_red: 1,
                num_green: 2,
                num_blue: 6
            }
        );
        assert_eq!(
            parse_handful("2 green"),
            Handful {
                num_red: 0,
                num_green: 2,
                num_blue: 0
            }
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                handfuls: vec![
                    Handful {
                        num_red: 4,
                        num_green: 0,
                        num_blue: 3
                    },
                    Handful {
                        num_red: 1,
                        num_green: 2,
                        num_blue: 6
                    },
                    Handful {
                        num_red: 0,
                        num_green: 2,
                        num_blue: 0
                    },
                ]
            }
        );
    }

    #[test]
    fn test_get_sum_of_possible_game_ids() {
        let input = concat!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        );
        assert_eq!(get_sum_of_possible_game_ids(input, 12, 13, 14), 8);
    }

    #[test]
    fn test_get_sum_of_power_of_min_possible_sets() {
        let input = concat!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        );
        assert_eq!(get_sum_of_power_of_min_possible_sets(input), 2286);
    }
}
