use regex::Regex;

advent_of_code::solution!(2);

enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
}
#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(input: String) -> Self {
        let splitted = input.split(":").collect::<Vec<&str>>();
        let id = splitted
            .get(0)
            .unwrap()
            .replace("Game ", "")
            .parse::<u32>()
            .unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let game = splitted
            .get(1)
            .unwrap()
            .split(";")
            .map(|v| {
                v.split(",")
                    .map(|w| {
                        let mut splitted = w.split_whitespace();
                        let first = splitted.next().unwrap().parse::<u32>().unwrap();
                        let second = splitted.next().unwrap();

                        return match second {
                            "red" => Cube::Red(first),
                            "blue" => Cube::Blue(first),
                            "green" => Cube::Green(first),
                            _ => unreachable!(),
                        };
                    })
                    .collect::<Vec<Cube>>()
            })
            .for_each(|v| {
                v.iter().for_each(|w| match *w {
                    Cube::Red(amount) => {
                        if amount > red {
                            red = amount
                        }
                    }
                    Cube::Blue(amount) => {
                        if amount > blue {
                            blue = amount
                        }
                    }
                    Cube::Green(amount) => {
                        if amount > green {
                            green = amount
                        }
                    }
                })
            });

        Self {
            id: id,
            red: red,
            green: green,
            blue: blue,
        }
    }
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let result: Vec<Game> = input
        .lines()
        .map(|v: &str| Game::new(v.to_string()))
        .collect::<Vec<_>>();

    let possible = result
        .iter()
        .filter(|v| v.blue <= MAX_BLUE && v.red <= MAX_RED && v.green <= MAX_GREEN)
        .collect::<Vec<_>>();

    let total_possible_id = possible.iter().fold(0, |acc, curr| acc + curr.id);
    println!("{}", total_possible_id);
    Some(total_possible_id)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: Vec<Game> = input
        .lines()
        .map(|v: &str| Game::new(v.to_string()))
        .collect::<Vec<_>>();

    let total_multiplied = result.iter().fold(0, |acc, curr| {
        return acc + (curr.red * curr.blue * curr.green);
    });
    println!("{}", total_multiplied);
    Some(total_multiplied)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2156));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(66909));
    }
}
