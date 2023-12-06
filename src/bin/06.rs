use regex::Regex;
advent_of_code::solution!(6);

fn ways_to_hold_button_to_win((time, distance): (u32, u32)) -> u32 {
    let mut ways = 0;
    for i in 0..time {
        let time_left = time - i;

        if i * time_left > distance {
            ways += 1;
        }
    }
    ways
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();

    let digits = input
        .lines()
        .map(|v| re.find_iter(v).map(|v| v.as_str()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let time = digits.get(0).unwrap();
    let distance = digits.get(1).unwrap();
    let len = time.len();
    let mut i = 0;

    let mut pairs = Vec::new();

    while i < len {
        let time = time.get(i).unwrap().parse::<u32>().unwrap();
        let distance = distance.get(i).unwrap().parse::<u32>().unwrap();
        pairs.push((time, distance));
        i += 1;
    }

    let result = pairs
        .into_iter()
        .map(|v| ways_to_hold_button_to_win(v))
        .reduce(|pre, acc| pre * acc)
        .unwrap();

    println!("{}", result);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();

    let digits = input
        .lines()
        .map(|v: &str| v.trim())
        .map(|v| re.find_iter(v).map(|v| v.as_str()).collect::<String>())
        .collect::<Vec<_>>();

    let time = digits.get(0).unwrap().parse::<u64>().unwrap();
    let distance = digits.get(1).unwrap().parse::<u64>().unwrap();

    let mut i = 0;

    loop {
        let time_left = time - i;
        if i * time_left > distance {
            break;
        }
        i += 1
    }

    let mut result = time - (i * 2);

    if time % 2 == 0 {
        result += 1
    }

    println!("{}", result);

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
