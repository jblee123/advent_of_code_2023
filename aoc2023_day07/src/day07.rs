use std::cmp::Ordering;

const CARD_TYPES: &str = "23456789TJQKA";
const CARD_TYPES_J: &str = "J23456789TQKA";

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
enum HandKind {
    FiveOfKind = 1,
    FourOfKind = 2,
    FullHouse = 3,
    ThreeOfKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
}

#[derive(Debug, PartialEq, /*Copy,*/ Clone)]
struct Hand {
    cards: String,
    kind: HandKind,
}

fn do_cmp_hand(hand1: &Hand, hand2: &Hand, card_types: &str) -> Ordering {
    if hand1.kind != hand2.kind {
        return hand1.kind.partial_cmp(&hand2.kind).unwrap();
    } else {
        let mut i = 0;
        while i < hand1.cards.len() {
            let card1 = hand1.cards.chars().nth(i).unwrap();
            let card2 = hand2.cards.chars().nth(i).unwrap();
            let card1_rank = card_types.find(card1).unwrap();
            let card2_rank = card_types.find(card2).unwrap();
            if card1_rank > card2_rank {
                return Ordering::Less;
            } else if card1_rank < card2_rank {
                return Ordering::Greater;
            }
            i += 1;
        }
        return Ordering::Equal;
    }
}

fn cmp_hand(hand1: &Hand, hand2: &Hand) -> Ordering {
    do_cmp_hand(hand1, hand2, CARD_TYPES)
}

fn cmp_hand_j(hand1: &Hand, hand2: &Hand) -> Ordering {
    do_cmp_hand(hand1, hand2, CARD_TYPES_J)
}

#[derive(Debug, PartialEq, /*Copy,*/ Clone)]
struct HandWithBid {
    hand: Hand,
    bid: u32,
}

fn parse_hand(hand_str: &str) -> Hand {
    let mut hand_chars = hand_str.chars().collect::<Vec<char>>();
    hand_chars.sort();

    let mut num_types = 1;
    let mut longest_type_run = 1;
    let mut current_type_run = 1;
    let mut idx = 1 as usize;
    while idx < hand_chars.len() {
        if hand_chars[idx - 1] != hand_chars[idx] {
            longest_type_run = longest_type_run.max(current_type_run);
            current_type_run = 1;
            num_types += 1;
        } else {
            current_type_run += 1;
        }

        idx += 1;
    }

    longest_type_run = longest_type_run.max(current_type_run);

    let kind = {
        if longest_type_run == 5 {
            HandKind::FiveOfKind
        } else if longest_type_run == 4 {
            HandKind::FourOfKind
        } else if longest_type_run == 3 {
            if num_types == 2 {
                HandKind::FullHouse
            } else {
                HandKind::ThreeOfKind
            }
        } else if longest_type_run == 2 {
            if num_types == 3 {
                HandKind::TwoPair
            } else {
                HandKind::OnePair
            }
        } else {
            HandKind::HighCard
        }
    };

    Hand {
        cards: hand_str.to_string(),
        kind: kind,
    }
}

fn parse_hand_with_bid(s: &str) -> HandWithBid {
    let mut parts = s.split(' ');
    let hand_str = parts.next().unwrap();
    let bid_str = parts.next().unwrap();

    HandWithBid {
        hand: parse_hand(hand_str),
        bid: bid_str.parse().unwrap(),
    }
}

fn parse_all_hands(s: &str) -> Vec<HandWithBid> {
    s.lines()
        .map(|line| parse_hand_with_bid(line))
        .collect::<Vec<HandWithBid>>()
}

fn calc_winnings(sorted_hands_with_bids: &Vec<HandWithBid>) -> u32 {
    let mut winnings = 0 as u32;
    sorted_hands_with_bids
        .iter()
        .enumerate()
        .for_each(|(idx, hand)| {
            winnings += (idx as u32 + 1) * hand.bid;
        });

    winnings
}

pub fn get_winnings(s: &str) -> u32 {
    let mut sorted_hands_with_bids = parse_all_hands(s);
    sorted_hands_with_bids.sort_by(|a, b| cmp_hand(&a.hand, &b.hand).reverse());
    calc_winnings(&sorted_hands_with_bids)
}

fn use_joker(hand: &Hand) -> Hand {
    let mut new_hand = hand.clone();

    let num_jokers = hand.cards.chars().filter(|c| *c == 'J').count();
    if hand.kind == HandKind::FiveOfKind {
        // do nothing -- can't improve
    } else if hand.kind == HandKind::FourOfKind {
        if (num_jokers == 4) || (num_jokers == 1) {
            new_hand.kind = HandKind::FiveOfKind;
        }
    } else if hand.kind == HandKind::FullHouse {
        if (num_jokers == 3) || (num_jokers == 2) {
            new_hand.kind = HandKind::FiveOfKind;
        }
    } else if hand.kind == HandKind::ThreeOfKind {
        if (num_jokers == 3) || (num_jokers == 1) {
            new_hand.kind = HandKind::FourOfKind;
        }
    } else if hand.kind == HandKind::TwoPair {
        if num_jokers == 2 {
            new_hand.kind = HandKind::FourOfKind;
        } else if num_jokers == 1 {
            new_hand.kind = HandKind::FullHouse;
        }
    } else if hand.kind == HandKind::OnePair {
        if (num_jokers == 2) || (num_jokers == 1) {
            new_hand.kind = HandKind::ThreeOfKind;
        }
    } else {
        if num_jokers == 1 {
            new_hand.kind = HandKind::OnePair;
        }
    }

    new_hand
}

pub fn get_winnings_with_jokers(s: &str) -> u32 {
    let mut sorted_hands_with_bids = parse_all_hands(s)
        .iter()
        .map(|hand_with_bid| HandWithBid {
            hand: use_joker(&hand_with_bid.hand),
            bid: hand_with_bid.bid,
        })
        .collect::<Vec<HandWithBid>>();
    sorted_hands_with_bids.sort_by(|a, b| cmp_hand_j(&a.hand, &b.hand).reverse());
    calc_winnings(&sorted_hands_with_bids)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "32T3K 765\n",
        "T55J5 684\n",
        "KK677 28\n",
        "KTJJT 220\n",
        "QQQJA 483\n",
    );

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            parse_hand("AAAAA"),
            Hand {
                cards: "AAAAA".to_string(),
                kind: HandKind::FiveOfKind,
            }
        );
        assert_eq!(
            parse_hand("AA8AA"),
            Hand {
                cards: "AA8AA".to_string(),
                kind: HandKind::FourOfKind,
            }
        );
        assert_eq!(
            parse_hand("TTT98"),
            Hand {
                cards: "TTT98".to_string(),
                kind: HandKind::ThreeOfKind,
            }
        );
        assert_eq!(
            parse_hand("23432"),
            Hand {
                cards: "23432".to_string(),
                kind: HandKind::TwoPair,
            }
        );
        assert_eq!(
            parse_hand("A23A4"),
            Hand {
                cards: "A23A4".to_string(),
                kind: HandKind::OnePair,
            }
        );
        assert_eq!(
            parse_hand("23456"),
            Hand {
                cards: "23456".to_string(),
                kind: HandKind::HighCard,
            }
        );
        assert_eq!(
            parse_hand("32T3K"),
            Hand {
                cards: "32T3K".to_string(),
                kind: HandKind::OnePair,
            }
        );
        assert_eq!(
            parse_hand("KK677"),
            Hand {
                cards: "KK677".to_string(),
                kind: HandKind::TwoPair,
            }
        );
        assert_eq!(
            parse_hand("KTJJT"),
            Hand {
                cards: "KTJJT".to_string(),
                kind: HandKind::TwoPair,
            }
        );
        assert_eq!(
            parse_hand("QQQJA"),
            Hand {
                cards: "QQQJA".to_string(),
                kind: HandKind::ThreeOfKind,
            }
        );
    }

    #[test]
    fn test_parse_hand_with_bid() {
        assert_eq!(
            parse_hand_with_bid("QQQJA 483"),
            HandWithBid {
                hand: Hand {
                    cards: "QQQJA".to_string(),
                    kind: HandKind::ThreeOfKind,
                },
                bid: 483,
            }
        );
    }

    #[test]
    fn test_get_winnings() {
        assert_eq!(get_winnings(SAMPLE_INPUT), 6440);
    }

    #[test]
    fn test_get_winnings_with_jokers() {
        assert_eq!(get_winnings_with_jokers(SAMPLE_INPUT), 5905);
    }
}
