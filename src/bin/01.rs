advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let result = line
                .chars()
                .filter(|char| char.is_numeric())
                .collect::<Vec<char>>();
            let first = result.get(0).unwrap().clone();
            let second = result.get(result.len() - 1).unwrap().clone();
            return [first, second]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
        })
        .sum::<u32>();

    Some(result)
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn convert_spelled_digit(input: &str) -> Vec<char> {
    let mut result = input.to_string();
    for digit in DIGITS {
        result = result.replace(
            digit,
            &format!("{}{}", digit, digit.chars().last().unwrap().to_string()),
        );
    }

    for digit in DIGITS {
        let as_digit = match digit {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!("not number get matched"),
        };

        result = result.replace(digit, as_digit);
    }

    return result
        .chars()
        .filter(|f| f.is_numeric())
        .collect::<Vec<char>>();
}

fn get_first_last<T>(slice: &[T]) -> Option<[T; 2]>
where
    T: Clone,
{
    let first = slice.get(0)?.clone();
    let last = slice.get(slice.len() - 1)?.clone();
    Some([first, last])
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|v| {
            let digit = convert_spelled_digit(v);
            let first_and_last = get_first_last(&digit).unwrap();
            return first_and_last
                .iter()
                .collect::<String>()
                .as_str()
                .parse::<u32>()
                .unwrap();
        })
        .sum::<u32>();
    println!("{:?}", result);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(55607));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(55291));
    }
}
