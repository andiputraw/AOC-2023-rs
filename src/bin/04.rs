use std::collections::HashMap;

advent_of_code::solution!(4);

struct CardGame<'a> {
    winning: Vec<&'a str>,
    mine: Vec<&'a str>,
}

impl<'a> CardGame<'a> {
    fn new(input: &'a str) -> Self {
        let splitted = input
            .split("|")
            .map(|v| v.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<_>>();

        let winning = splitted.get(0).unwrap().to_vec();
        let mine = splitted.get(1).unwrap().to_vec();

        CardGame { winning, mine }
    }

    fn calculate_winner(self: &'a Self) -> u32 {
        let mut map = HashMap::new();

        for win in self.winning.iter() {
            map.insert(win, 0);
        }

        for mine in self.mine.iter() {
            if let Some(val) = map.get(mine) {
                map.insert(mine, val + 1);
            }
        }

        let mut result = 0;
        for (_, val) in map.iter() {
            result += val;
        }

        println!("{}", result);
        if result == 0 {
            result
        } else {
            2_u32.pow(result - 1)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|v| v.split(":").last().unwrap())
        .map(|v| CardGame::new(v).calculate_winner())
        .sum::<u32>();

    println!("{}", result);
    None
}

struct CopiedCalc {
    copied_card: HashMap<u32, u32>,
}
struct TrueCardGame<'a> {
    winning: Vec<&'a str>,
    mine: Vec<&'a str>,
}

impl<'a> TrueCardGame<'a> {
    fn new(input: &'a str) -> Self {
        let splitted = input
            .split("|")
            .map(|v| v.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<_>>();

        let winning = splitted.get(0).unwrap().to_vec();
        let mine = splitted.get(1).unwrap().to_vec();

        TrueCardGame { winning, mine }
    }

    fn get_matching(self: &'a Self) -> u32 {
        let mut map = HashMap::new();

        for win in self.winning.iter() {
            map.insert(win, 0);
        }

        for mine in self.mine.iter() {
            if let Some(val) = map.get(mine) {
                map.insert(mine, val + 1);
            }
        }

        let mut result = 0;
        for (_, val) in map.iter() {
            result += val;
        }

        result
    }
}

impl CopiedCalc {
    fn calculate_copy(self: &mut Self, game_id: u32, card: TrueCardGame) -> u32 {
        let copy = match self.copied_card.get(&game_id) {
            Some(v) => v,
            None => {
                self.copied_card.insert(game_id, 1);
                &1
            }
        }
        .clone();
        let result = card.get_matching();
        println!("gameId: {} copy: {} result: {}", game_id, copy, result);
        for i in (game_id + 1)..=(result + game_id) {
            let current_copy = self.copied_card.get(&i).unwrap_or(&1);
            println!("    Game Id: {} current_copy: {}", i, current_copy);
            self.copied_card.insert(i, (current_copy + (1 * copy)));
        }
        0
    }
    fn get_all_copy(self: Self) -> u32 {
        println!("{:?}", self.copied_card);
        self.copied_card.values().sum::<u32>()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|v| v.split(":").last().unwrap())
        .map(|v| TrueCardGame::new(v));

    let mut global_card = CopiedCalc {
        copied_card: HashMap::new(),
    };

    for (index, el) in result.enumerate() {
        global_card.calculate_copy((index).try_into().unwrap(), el);
    }

    println!("Total Card: {}", global_card.get_all_copy());

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
