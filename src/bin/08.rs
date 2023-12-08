use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(8);

struct Maps<'a> {
    instruction: Vec<char>,
    maps: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Maps<'a> {
    fn new(input: &'a str) -> Self {
        let lines: Vec<_> = input.lines().collect();

        let mut map: HashMap<&'a str, (&'a str, &'a str)> = HashMap::new();

        for line in lines {
            let parts: Vec<&'a str> = line.split('=').collect();

            if parts.len() == 2 {
                let key = parts[0].trim();
                let values = parts[1]
                    .trim_matches(|c| c == '(' || c == ')' || c == ' ')
                    .split(',')
                    .collect::<Vec<&'a str>>();

                if values.len() == 2 {
                    let tuple = (values[0].trim(), values[1].trim());
                    map.insert(key, tuple);
                }
            }
        }

        Maps {
            instruction: Vec::new(),
            maps: map,
        }
    }

    fn set_instruction(&mut self, input: &str) {
        let inst = input.chars().collect::<Vec<char>>();
        self.instruction = inst;
    }

    fn step_to_zzz(&self) -> u32 {
        let mut key = "AAA";
        let mut now_step = 0;
        loop {
            if key == "ZZZ" {
                break;
            }
            let (left, right) = self.maps.get(key).unwrap();
            let index = now_step % (self.instruction.len());
            let direction = self.instruction.get(index).unwrap();

            if direction == &'L' {
                key = left;
                now_step += 1;
            } else {
                key = right;
                now_step += 1;
            }
        }
        return now_step as u32;
    }

    fn get_last_a_keys(&self) -> Vec<&str> {
        let keys = self.maps.clone();

        keys.into_keys()
            .filter(|v| v.chars().last().unwrap() == 'A')
            .collect()
    }

    fn is_key_end_with_z(input: &str) -> bool {
        input.chars().last().unwrap() == 'Z'
    }

    fn step_from_last_a_to_last_z(&self) -> u64 {
        let mut keys = self.get_last_a_keys();
        let mut result: Vec<u64> = Vec::with_capacity(keys.len());

        for (i, key) in keys.iter().enumerate() {
            let mut key = key;
            let mut now_step = 0;
            loop {
                if Self::is_key_end_with_z(*key) {
                    break;
                }
                let (left, right) = self.maps.get(key).unwrap();
                let index = now_step % (self.instruction.len());
                let direction = self.instruction.get(index).unwrap();

                if direction == &'L' {
                    key = left;
                    now_step += 1;
                } else {
                    key = right;
                    now_step += 1;
                }
            }
            result.push(now_step as u64);
        }
        println!("{:?}", result);

        let kpk = calculate_kpk(&result);

        kpk
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.lines().collect::<VecDeque<&str>>();
    let inst = input_iter.pop_front().unwrap();
    let input = input_iter.as_slices().0.join("\n");
    let mut maps = Maps::new(&input);
    maps.set_instruction(inst);

    let result = maps.step_to_zzz();

    println!("{}", result);

    None
}
fn fpb(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        fpb(b, a % b)
    }
}

fn kpk(a: u64, b: u64) -> u64 {
    (a * b) / fpb(a, b)
}

fn calculate_kpk(numbers: &[u64]) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = kpk(result, num);
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_iter = input.lines().collect::<VecDeque<&str>>();
    let inst = input_iter.pop_front().unwrap();
    let input = input_iter.as_slices().0.join("\n");
    let mut maps = Maps::new(&input);
    maps.set_instruction(inst);

    let result = maps.step_from_last_a_to_last_z();

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
