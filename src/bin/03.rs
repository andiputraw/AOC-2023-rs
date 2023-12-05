advent_of_code::solution!(3);

struct SchematicLexer {
    x: usize,
    y: usize,
    x_max: usize,
    schematic: Vec<Vec<char>>,
}

impl SchematicLexer {
    fn new(input: &str) -> Self {
        let schematic = input
            .lines()
            .map(|v| v.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let x_max = schematic.get(0).unwrap().len();

        Self {
            x: 0,
            y: 0,
            x_max,
            schematic: schematic,
        }
    }

    fn get_exact_coordinate(&self, x: usize, y: usize) -> Option<char> {
        if let Some(line) = self.schematic.get(y) {
            if let Some(symbol) = line.get(x) {
                Some(*symbol)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_symbol_on_bound(&self, x1: usize, x2: usize, y: usize) -> bool {
        let box_x1 = x1.saturating_sub(1);
        let box_x2 = x2 + 1;

        let mut i = box_x1;
        let mut is_adj = false;
        'outer: for y in (y.saturating_sub(1))..=(y + 1) {
            while i <= box_x2 {
                if let Some(symbol) = self.get_exact_coordinate(i, y) {
                    if !is_symbol(&symbol) {
                        i += 1;
                        continue;
                    }
                    is_adj = true;
                    break 'outer;
                } else {
                    i += 1;
                    continue;
                }
            }
            i = box_x1;
        }

        is_adj
    }

    fn get_numeric(&mut self) -> (usize, usize) {
        let vec = self.schematic.get(self.y).unwrap();
        let start = self.x;
        while let Some(symbol) = vec.get(self.x) {
            if symbol.is_numeric() {
                self.x += 1;
            } else {
                break;
            }
        }

        (start, self.x - 1)
    }

    fn trim_until_numeric(&mut self) -> (usize, usize) {
        let mut x1_x2 = (0, 0);

        'outer: while self.y < self.schematic.len() {
            while self.x < self.x_max {
                if let Some(symbol) = self.get_exact_coordinate(self.x, self.y) {
                    if symbol.is_numeric() {
                        x1_x2 = self.get_numeric();
                        break 'outer;
                    }
                }

                self.x += 1;
            }
            self.x = 0;
            self.y += 1;
        }

        return x1_x2;
    }

    fn get_char(&self, x1: usize, x2: usize, y: usize) -> u32 {
        let vec = self.schematic.get(y).unwrap();

        let str = vec.get(x1..=x2).unwrap().iter().collect::<String>();
        println!("{}", str);
        str.parse::<u32>().unwrap()
    }

    fn _next(&mut self) -> Option<u32> {
        while self.y < self.schematic.len() {
            let (x1, x2) = self.trim_until_numeric();
            println!("x1: {} | x2: {}", x1, x2);
            if self.is_symbol_on_bound(x1, x2, self.y) {
                return Some(self.get_char(x1, x2, self.y));
            }
        }

        None
    }
}

impl Iterator for SchematicLexer {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        return self._next();
    }
}

fn is_symbol(ch: &char) -> bool {
    return !ch.is_numeric() && *ch != '.';
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = SchematicLexer::new(input);

    let result = schematic.sum::<u32>();

    println!("{}", result);

    None
}

struct GearLexer {
    x: usize,
    y: usize,
    x_max: usize,
    schematic: Vec<Vec<char>>,
}

impl GearLexer {
    fn new(input: &str) -> Self {
        let schematic = input
            .lines()
            .map(|v| v.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let x_max = schematic.get(0).unwrap().len();

        Self {
            x: 0,
            y: 0,
            x_max,
            schematic: schematic,
        }
    }

    fn get_exact_coordinate(&self, x: usize, y: usize) -> Option<char> {
        if let Some(line) = self.schematic.get(y) {
            if let Some(symbol) = line.get(x) {
                Some(*symbol)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_number_on_bound(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let box_x1 = x.saturating_sub(1);
        let box_x2 = x + 1;

        let mut i = box_x1;
        let mut vec = Vec::new();
        'outer: for y in (y.saturating_sub(1))..=(y + 1) {
            while i <= box_x2 {
                if let Some(symbol) = self.get_exact_coordinate(i, y) {
                    if !symbol.is_numeric() {
                        i += 1;
                        continue;
                    }
                    vec.push((i, y));
                    break 'outer;
                } else {
                    i += 1;
                    continue;
                }
            }
            i = box_x1;
        }

        vec
    }

    fn get_numeric(&mut self) -> (usize, usize) {
        let vec = self.schematic.get(self.y).unwrap();
        let start = self.x;
        while let Some(symbol) = vec.get(self.x) {
            if symbol.is_numeric() {
                self.x += 1;
            } else {
                break;
            }
        }

        (start, self.x - 1)
    }

    fn trim_until_gear(&mut self) -> (usize) {
        'outer: while self.y < self.schematic.len() {
            while self.x < self.x_max {
                if let Some(symbol) = self.get_exact_coordinate(self.x, self.y) {
                    if symbol == '*' {
                        break 'outer;
                    }
                }

                self.x += 1;
            }
            self.x = 0;
            self.y += 1;
        }

        return self.x;
    }

    fn peek_number() {}

    fn get_number(&self, vec: Vec<(usize, usize)>) -> u32 {
        for (x, y) in vec {
            let line = self.schematic.get(y).unwrap();
        }

        let str = vec.get(x1..=x2).unwrap().iter().collect::<String>();
        println!("{}", str);
        str.parse::<u32>().unwrap()
    }

    fn _next(&mut self) -> Option<u32> {
        while self.y < self.schematic.len() {
            let x = self.trim_until_gear();
            println!("Gear is on coordinate ({} - {})", x, self.y);
            let number_coordinate = self.is_number_on_bound(x, self.y);
        }

        None
    }
}

impl Iterator for GearLexer {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        return self._next();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
