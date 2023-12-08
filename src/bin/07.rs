use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug)]
struct CardGame {
    bid: u32,
    card: Vec<char>,
}

impl CardGame {
    fn new(input: &str) -> Self {
        let mut str_iter = input.split_whitespace();
        let card: Vec<char> = str_iter.next().unwrap().chars().collect();
        let bid = str_iter.next().unwrap().parse::<u32>().unwrap();

        CardGame { bid, card }
    }
}

fn card_value(a: &char) -> u32 {
    match a {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("illegal card found"),
    }
}

fn card_value_joker(a: &char) -> u32 {
    match a {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("illegal card found"),
    }
}

fn card_order(a: &char, b: &char) -> Ordering {
    let a = card_value(a);
    let b = card_value(b);

    if a > b {
        Ordering::Greater
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
#[derive(Debug)]
enum CardType {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

impl CardType {
    fn to_integer(self) -> u32 {
        match self {
            CardType::Five => 7,
            CardType::Four => 6,
            CardType::Full => 5,
            CardType::Three => 4,
            CardType::TwoPair => 3,
            CardType::OnePair => 2,
            CardType::HighCard => 1,
        }
    }
}

fn get_card_type(bucket: &mut HashMap<&char, u32>) -> CardType {
    let mut is_three_pair = false;
    let mut is_two_pair = false;
    let mut card_pair = CardType::HighCard;
    let len = bucket.len();
    let mut iteration = 0;
    for (_, v) in bucket.drain() {
        if v == 5 {
            card_pair = CardType::Five;
            break;
        } else if v == 4 {
            card_pair = CardType::Four;
            break;
        } else if v == 3 {
            if is_two_pair {
                card_pair = CardType::Full;
                break;
            }
            is_three_pair = true;
        } else if v == 2 {
            if is_three_pair {
                card_pair = CardType::Full;
                break;
            }
            if is_two_pair {
                card_pair = CardType::TwoPair;
                break;
            }
            is_two_pair = true;
        }
        if iteration == len - 1 {
            if is_three_pair {
                card_pair = CardType::Three;
                break;
            }

            if is_two_pair {
                card_pair = CardType::OnePair;
                break;
            }
        }

        card_pair = CardType::HighCard;
        iteration += 1;
    }

    card_pair
}

fn order_card(a: &CardGame, b: &CardGame) -> Ordering {
    let mut bucket: HashMap<&char, u32> = HashMap::with_capacity(5);
    let mut len: usize;
    for card in a.card.iter() {
        let value = bucket.get(card).unwrap_or(&0);
        bucket.insert(card, *value + 1);
    }

    let card_type_a = get_card_type(&mut bucket);

    for card in b.card.iter() {
        let value = bucket.get(card).unwrap_or(&0);
        bucket.insert(card, *value + 1);
    }

    let card_type_b = get_card_type(&mut bucket);

    let score_a = card_type_a.to_integer();
    let score_b = card_type_b.to_integer();

    return if score_a > score_b {
        Ordering::Greater
    } else if score_a < score_b {
        Ordering::Less
    } else {
        let mut i = 0;
        let mut result = Ordering::Equal;
        while i <= a.card.len() {
            let a = a.card.get(i).unwrap();
            let b = b.card.get(i).unwrap();

            let order = card_order(a, b);
            if order == Ordering::Equal {
                i += 1;
                continue;
            }
            result = order;
            break;
        }

        result
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = input
        .lines()
        .map(|v: &str| CardGame::new(v))
        .collect::<Vec<CardGame>>();

    result.sort_by(order_card);

    let mut total_bid = 0;

    for (i, card_game) in result.into_iter().enumerate() {
        total_bid += card_game.bid * (i as u32 + 1)
    }

    println!("{:#?}", total_bid);
    None
}

fn get_card_type_joker(bucket: &mut HashMap<&char, u32>) -> CardType {
    let card_pair;
    let any = bucket.remove(&'J').unwrap_or(0);

    let mut tmp = bucket.drain().map(|v| v.1).collect::<Vec<u32>>();
    tmp.sort();
    tmp.reverse();

    println!("before tmp: {:?}", tmp);

    let first = tmp.get(0).unwrap_or(&0);
    if tmp.is_empty() {
        tmp.push(first + any);
    } else {
        tmp[0] = first + any
    }
    println!("after tmp: {:?}", tmp);

    match tmp.as_slice() {
        [1, 1, 1, 1, 1] => card_pair = CardType::HighCard,
        [2, 1, 1, 1] => card_pair = CardType::OnePair,
        [2, 2, 1] => card_pair = CardType::TwoPair,
        [3, 1, 1] => card_pair = CardType::Three,
        [3, 2] => card_pair = CardType::Full,
        [4, 1] => card_pair = CardType::Four,
        [5] => card_pair = CardType::Five,
        _ => unreachable!(),
    }

    card_pair
}

fn card_order_joker(a: &char, b: &char) -> Ordering {
    let a = card_value_joker(a);
    let b = card_value_joker(b);

    if a > b {
        Ordering::Greater
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn order_card_joker(a: &CardGame, b: &CardGame) -> Ordering {
    let mut bucket: HashMap<&char, u32> = HashMap::with_capacity(5);
    let mut len: usize;
    for card in a.card.iter() {
        let value = bucket.get(card).unwrap_or(&0);
        bucket.insert(card, *value + 1);
    }
    let card_type_a = get_card_type_joker(&mut bucket);

    for card in b.card.iter() {
        let value = bucket.get(card).unwrap_or(&0);
        bucket.insert(card, *value + 1);
    }
    let card_type_b = get_card_type_joker(&mut bucket);

    let score_a = card_type_a.to_integer();
    let score_b = card_type_b.to_integer();

    return if score_a > score_b {
        Ordering::Greater
    } else if score_a < score_b {
        Ordering::Less
    } else {
        let mut i = 0;
        let mut result = Ordering::Equal;
        while i <= a.card.len() {
            let a = a.card.get(i).unwrap();
            let b = b.card.get(i).unwrap();

            let order = card_order_joker(a, b);
            if order == Ordering::Equal {
                i += 1;
                continue;
            }
            result = order;
            break;
        }

        result
    };
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = input
        .lines()
        .map(|v: &str| CardGame::new(v))
        .collect::<Vec<CardGame>>();

    result.sort_by(order_card_joker);

    // println!("{:#?}", result);

    let mut total_bid = 0;

    for (i, card_game) in result.into_iter().enumerate() {
        total_bid += card_game.bid * (i as u32 + 1)
    }

    println!("{:#?}", total_bid);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
