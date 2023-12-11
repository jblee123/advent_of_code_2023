#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

fn parse_card(s: &str) -> Card {
    let colon_idx = s.find(':').unwrap();
    let pipe_idx = s.find('|').unwrap();
    let id = s[5..colon_idx].trim().parse::<u32>().unwrap();

    let winners_str = &s[colon_idx + 1..pipe_idx];
    let numbers_str = &s[pipe_idx + 1..];

    let winners = winners_str
        .split(' ')
        .filter(|part| !part.is_empty())
        .map(|part| part.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let numbers = numbers_str
        .split(' ')
        .filter(|part| !part.is_empty())
        .map(|part| part.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Card {
        id: id,
        winners: winners,
        numbers: numbers,
    }
}

fn get_num_matches(card: &Card) -> u32 {
    let mut winners = card.winners.clone();
    let mut numbers = card.numbers.clone();
    winners.sort();
    numbers.sort();

    let mut idx1 = 0 as usize;
    let mut idx2 = 0 as usize;
    let mut matches = 0;

    let max_idx1 = winners.len() - 1;
    let max_idx2 = numbers.len() - 1;

    while idx1 < winners.len() && idx2 < numbers.len() {
        if winners[idx1] == numbers[idx2] {
            matches += 1;
            idx1 += 1;
            idx2 += 1;
        } else {
            if idx1 == max_idx1 {
                idx2 += 1
            } else if idx2 == max_idx2 {
                idx1 += 1
            } else if winners[idx1] < numbers[idx2] {
                idx1 += 1;
            } else {
                idx2 += 1;
            }
        }
    }

    matches
}

pub fn get_points_for_cards(s: &str) -> u32 {
    s.lines()
        .map(|line| parse_card(line))
        .map(|card| get_num_matches(&card))
        .map(|num_matches| {
            if num_matches > 0 {
                (2 as u32).pow(num_matches - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn get_num_cards_after_rewinning(s: &str) -> u32 {
    let cards = s
        .lines()
        .map(|line| parse_card(line))
        .collect::<Vec<Card>>();
    let mut card_counts = vec![1; cards.len() + 1];
    card_counts[0] = 0;

    for card in cards {
        let num_matches = get_num_matches(&card);
        let num_of_current_card = card_counts[card.id as usize];
        for won_card_id in (card.id + 1)..=(card.id + num_matches) {
            card_counts[won_card_id as usize] += num_of_current_card;
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        assert_eq!(
            parse_card(input),
            Card {
                id: 3,
                winners: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            }
        );

        let input = "Card   3: 90 28  1 14 41 83 24 59 55  6 | 29 15 72 47 32 96 73 76 52 20 53 26 78 39 85 92  3 67 51 75 64 54 99 65 22";
        assert_eq!(
            parse_card(input),
            Card {
                id: 3,
                winners: vec![90, 28, 1, 14, 41, 83, 24, 59, 55, 6],
                numbers: vec![
                    29, 15, 72, 47, 32, 96, 73, 76, 52, 20, 53, 26, 78, 39, 85, 92, 3, 67, 51, 75,
                    64, 54, 99, 65, 22
                ],
            }
        );
    }

    #[test]
    fn test_get_num_matches() {
        let card = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(get_num_matches(&card), 4);

        let card = parse_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(get_num_matches(&card), 2);

        let card = parse_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(get_num_matches(&card), 2);

        let card = parse_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(get_num_matches(&card), 1);

        let card = parse_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(get_num_matches(&card), 0);

        let card = parse_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(get_num_matches(&card), 0);
    }

    #[test]
    fn test_get_points_for_cards() {
        let input = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
        );

        assert_eq!(get_points_for_cards(input), 13);
    }

    #[test]
    fn test_get_num_cards_after_rewinning() {
        let input = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
        );

        assert_eq!(get_num_cards_after_rewinning(input), 30);
    }
}
