use std::io::Read;
use std::str::FromStr;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn get_input(filename: &str) -> Result<String, std::io::Error> {
    let mut out = String::new();

    let mut file = File::open(filename)?;
    file.read_to_string(&mut out)?;

    Ok(out)
}

fn inverse_captcha<P>(input: &str, peeker_fn: P) -> u32 
where P: Fn(usize, usize) -> usize
{
    let mut sum = 0;
    let mut peek_idx;
    for char_ind in input.char_indices() {
        let (idx, val) = char_ind;
        peek_idx = peeker_fn(idx, input.len());
        if peek_idx > input.len() - 1 {
            peek_idx = peek_idx - input.len();
        }

        if let Some(peek_val) = input.chars().nth(peek_idx) {
            if let Some(digit) = val.to_digit(10) {
                if peek_val == val {
                    sum += digit;
                }
            }
        }
    }
    sum 
}

fn corruption_checksum<P>(input: &str, collector_fn: P) -> u32 
where P: Fn(&Vec<u32>, &mut Vec<u32>) {
    let mut checksum: Vec<_> = Vec::new();
    for line in input.lines() {
        let v: Vec<u32> = line
            .split_whitespace()
            .map(|i| u32::from_str(i).unwrap())
            .collect();

        collector_fn(&v, &mut checksum);
    }    
    checksum.iter().sum() 
}

fn spiral_memory<P>(input: &str, value_fn: P) -> HashMap<u32, (i32, i32)> 
where P: Fn(u32, &HashMap<(i32, i32), u32>, i32, i32) -> u32 {
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

        for _ in 1..incr+1 {
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

fn main() {
    // match io::stdin().read_to_string(&mut input) {
    //    Ok(_) => {}
    //    Err(error) => println!("Must provide an input value: {}", error), 
    // }

    let mut input = get_input("data/day1.input").expect("Failed to read data/day1.input");;

    // Day 1 - Part One
    println!("Day 1 Part One = {}", inverse_captcha(&input, |idx, _| {
        idx + 1
    }));

    // Day 1 - Part Two
    println!("Day 1 Part Two = {}", inverse_captcha(&input, |idx, len| {
        idx + len / 2
    }));
    
    input = get_input("data/day2.input").expect("Failed to read data/day2.input");;

    println!("Day 2 Part One = {}", corruption_checksum(&input, |v, out| {
        out.push(v.iter().max().unwrap() - v.iter().min().unwrap());
    }));

    println!("Day 2 Part Two = {}", corruption_checksum(&input, |v, out| {
        for x in v.iter() {
            for y in v.iter() {
                if x != y && x % y == 0 {
                    out.push(x / y);
                } 
            } 
        }
    }));

    input = get_input("data/day3.input").expect("Failed to read data/day3.input");;

    {
        let mem = spiral_memory(&input, |v, _, _, _| v + 1);
        let value = u32::from_str(&input).unwrap();
        println!("Day 3 Part One = {}", manhatten_distance(&mem, 1, value)); 
    }

    {
        let mem = spiral_memory(&input, |v, coord_map, x, y| {
            if v == 0 {
                1
            } else {
                let directions: [(i32, i32); 9] = [
                    (0, 0), // self
                    (0, -1), // above
                    (1, -1), // upper right
                    (1, 0),  // right
                    (1, 1),  // lower right
                    (0, 1),  // below
                    (-1, 1),  // lower left
                    (-1, 0),  // left
                    (-1, -1),  // upper left
                ];

                let dirs: Vec<u32> = directions.iter().map(|&d| {
                    coord_map.get(&(x + d.0, y + d.1)).unwrap_or(&0).clone()
                }).collect();

                dirs.iter().sum()
            }
        });

        let mut gt: Vec<&u32> = mem.keys().collect();
        gt.sort();
        println!("Day 3 Part Two = {:?}", gt.iter().last().unwrap());
    }

    {
        input = get_input("data/day4.input").expect("Failed to read data/day4.input");
        println!("Day 4 Part One = {:?}", high_entropy(&input, |w| String::from(w)));
        println!("Day 4 Part Two = {:?}", high_entropy(&input, |w| {
            let mut chars: Vec<_> = w.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            String::from_iter(chars)
        }));
    }

    {
        input = get_input("data/day5.input").expect("Failed to read data/day5.input");
        println!("Day 5 Part One = {:?}", twisty(&input, |v| v + 1));
        println!("Day 5 Part One = {:?}", twisty(&input, |v| {
            if v < 3 {
                v + 1
            } else {
                v - 1
            }
        }));
    }
}
