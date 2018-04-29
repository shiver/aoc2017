#![feature(test)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::str::FromStr;

fn get_input(filename: &str) -> Result<String, std::io::Error> {
    let mut out = String::new();

    let mut file = File::open(filename)?;
    file.read_to_string(&mut out)?;

    Ok(out)
}

fn inverse_captcha<P>(input: &String, peeker_fn: P) -> u32
    where P: Fn(usize, usize) -> usize
{
    input
        .char_indices()
        .fold(0, |acc, (idx, val)| {
            let peeker_idx = peeker_fn(idx, input.len());
            let peeker_idx = if peeker_idx > input.len() - 1 {
                peeker_idx - input.len()
            } else {
                peeker_idx
            };

            if let Some(peeked) = input.chars().nth(peeker_idx) {
                if let Some(digit_val) = val.to_digit(10) {
                    if let Some(peeked_digit) = peeked.to_digit(10) {
                        if digit_val == peeked_digit {
                            return acc + peeked_digit;
                        }
                    }
                }
            }

            acc
        })
}

fn corruption_checksum<P>(input: &str, collector_fn: P) -> u32
    where P: Fn(&Vec<u32>, &mut Vec<u32>)
{
    let mut checksum: Vec<_> = Vec::new();
    for line in input.lines() {
        let v: Vec<u32> = line.split_whitespace()
            .map(|i| u32::from_str(i).unwrap())
            .collect();

        collector_fn(&v, &mut checksum);
    }
    checksum.iter().sum()
}

fn spiral_memory<P>(input: &str, value_fn: P) -> HashMap<u32, (i32, i32)>
    where P: Fn(u32, &HashMap<(i32, i32), u32>, i32, i32) -> u32
{
    // 1 1 2 2 3 3 4 4
    // +x -y -x +y

    let mutators: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut incr = 1;

    let max_value = u32::from_str(input).unwrap();
    let mut value_map = HashMap::new();
    let mut coord_map = HashMap::new();

    let mut mut_it = mutators.iter().cycle();
    let mut idx = 1;
    let mut value = 0;
    while value <= max_value {
        let mutator = mut_it.next().unwrap();

        for _ in 1..incr + 1 {
            value = value_fn(value, &coord_map, x, y);

            value_map.insert(value, (x, y));
            coord_map.insert((x, y), value);
            x += mutator.0;
            y += mutator.1;

            if value > max_value {
                break;
            }
        }

        if idx % 2 == 0 {
            incr += 1
        }
        idx += 1;
    }

    value_map
}

fn manhatten_distance(map: &HashMap<u32, (i32, i32)>, p1: u32, p2: u32) -> i32 {
    let _p1 = map.get(&p1).unwrap();
    let _p2 = map.get(&p2).unwrap();

    ((_p1.0 - _p2.0).abs() + (_p1.1 - _p2.1).abs())
}

fn high_entropy<P>(input: &str, mutator_fn: P) -> u32
    where P: Fn(&str) -> String
{
    let mut set = HashSet::new();
    let mut counter = 0;

    for line in input.lines() {
        set.clear();

        for word in line.split_whitespace() {
            let mutated_word = mutator_fn(word);
            if set.contains(&mutated_word) {
                break;
            }
            set.insert(mutated_word);
        }

        if line.split_whitespace().count() == set.len() {
            counter += 1;
        }
    }

    counter
}

fn twisty<P>(input: &str, mutator_fn: P) -> u32
    where P: Fn(i32) -> i32
{
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (idx, line) in input.lines().enumerate() {
        let value = i32::from_str(line).unwrap();
        map.insert(idx as i32, value);
    }

    let mut ip: i32 = 0;
    let mut counter = 0;
    while map.contains_key(&ip) {
        if let Some(instruction) = map.get_mut(&ip) {
            ip += *instruction;
            *instruction = mutator_fn(*instruction);
        }
        counter += 1;
    }
    counter
}

fn reallocation(input: &str) -> u32 {
    let mut bank: Vec<u32> = input
        .split_whitespace()
        .map(|v| u32::from_str(v).unwrap())
        .collect();
    let mut bank_patterns = HashSet::new();

    while !bank_patterns.contains(&bank.clone()) {
        let largest = bank.iter().max().unwrap();
        let mut bank_idx = bank.binary_search(&largest).unwrap();
        let it = bank.iter().skip(bank_idx).cycle();
        let mut acc = largest;
        for i in it {
            if acc <= &0 {
                break;
            }
        }

        println!("{:?}", bank);
        bank_patterns.insert(bank.clone());
    }
    // let mut bank_idx = bank.binary_search(bank.max());

    // for idx in bank.iter().cycle() {

    // }
    1
}

fn main() {
    // match io::stdin().read_to_string(&mut input) {
    //    Ok(_) => {}
    //    Err(error) => println!("Must provide an input value: {}", error),
    // }

    // {
    //     let input = get_input("data/day6.input").expect("Failed to read data/day6.input");
    //     println!("Day 6 Part One = {:?}", reallocation(&input));
    // }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use ::*;

    #[test]
    fn test_day1_part1() {
        assert!(inverse_captcha(&String::from("1122"), |idx, _| idx + 1) == 3);
        assert!(inverse_captcha(&String::from("1111"), |idx, _| idx + 1) == 4);
        assert!(inverse_captcha(&String::from("1234"), |idx, _| idx + 1) == 0);
        assert!(inverse_captcha(&String::from("91212129"), |idx, _| idx + 1) == 9);

        let input = get_input("data/day1.input").expect("Failed to read data/day1.input");
        assert!(inverse_captcha(&input, |idx, _| idx + 1) == 1393);
    }

    #[test]
    fn test_day1_part2() {
        assert!(inverse_captcha(&String::from("1212"), |idx, len| idx + len / 2) == 6);
        assert!(inverse_captcha(&String::from("1221"), |idx, len| idx + len / 2) == 0);
        assert!(inverse_captcha(&String::from("123425"), |idx, len| idx + len / 2) == 4);
        assert!(inverse_captcha(&String::from("123123"), |idx, len| idx + len / 2) == 12);
        assert!(inverse_captcha(&String::from("12131415"), |idx, len| idx + len / 2) == 4);

        let input = get_input("data/day1.input").expect("Failed to read data/day1.input");
        assert!(inverse_captcha(&input, |idx, len| idx + len / 2) == 1292)
    }

    #[test]
    fn test_day2_part1() {
        let input = get_input("data/day2.input").expect("Failed to read data/day2.input");
        assert!(corruption_checksum(&input, |v, out| {
            out.push(v.iter().max().unwrap() - v.iter().min().unwrap());
        }) == 53978)
    }

    #[test]
    fn test_day2_part2() {
        let input = get_input("data/day2.input").expect("Failed to read data/day2.input");
        assert!(corruption_checksum(&input, |v, out| for x in v.iter() {
            for y in v.iter() {
                if x != y && x % y == 0 {
                    out.push(x / y);
                }
            }
        }) == 314)
    }

    #[test]
    fn test_day3_part1() {
        let input = get_input("data/day3.input").expect("Failed to read data/day3.input");

        let mem = spiral_memory(&input, |v, _, _, _| v + 1);
        let value = u32::from_str(&input).unwrap();

        assert!(manhatten_distance(&mem, 1, value) == 326);
    }

    #[test]
    fn test_day3_part2() {
        let input = get_input("data/day3.input").expect("Failed to read data/day3.input");
        let mem = spiral_memory(&input, |v, coord_map, x, y| {
            if v == 0 {
                1
            } else {
                let directions: [(i32, i32); 9] = [
                    (0, 0),   // self
                    (0, -1),  // above
                    (1, -1),  // upper right
                    (1, 0),   // right
                    (1, 1),   // lower right
                    (0, 1),   // below
                    (-1, 1),  // lower left
                    (-1, 0),  // left
                    (-1, -1), // upper left
                ];

                let dirs: Vec<u32> = directions
                    .iter()
                    .map(|&d| coord_map.get(&(x + d.0, y + d.1)).unwrap_or(&0).clone())
                    .collect();

                dirs.iter().sum()
            }
        });

        let mut gt: Vec<&u32> = mem.keys().collect();
        gt.sort();
        assert!(gt.last() == Some(&&363010));
    }

    #[test]
    fn test_day4_part1() {
        let input = get_input("data/day4.input").expect("Failed to read data/day4.input");
        assert!(high_entropy(&input, |w| String::from(w)) == 451);
    }

    #[test]
    fn test_day4_part2() {
        let input = get_input("data/day4.input").expect("Failed to read data/day4.input");
        assert!(high_entropy(&input, |w| {
            let mut chars: Vec<_> = w.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            String::from_iter(chars)
        }) == 223);
    }

    #[test]
    fn test_day5_part1() {
        let input = get_input("data/day5.input").expect("Failed to read data/day5.input");
        assert!(twisty(&input, |v| v + 1) == 339351);
    }

    #[test]
    fn test_day5_part2() {
        let input = get_input("data/day5.input").expect("Failed to read data/day5.input");
        assert!(twisty(&input, |v| if v < 3 { v + 1 } else { v - 1 }) == 24315397);
    }

    #[test]
    fn test_day6_part1() {
        let input = get_input("data/day6.input").expect("Failed to read data/day5.input");
        println!("Day 6 Part One = {:?}", reallocation(&input));
    }
}
