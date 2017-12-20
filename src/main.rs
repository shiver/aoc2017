use std::io::Read;
use std::str::FromStr;
use std::fs::File;
use std::collections::HashMap;

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

fn spiral_memory<P>(input: &str, value_fn: P) -> i32 
where P: Fn(u32, &HashMap<(i32, i32), u32>) -> u32 {
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
    let mut value = 1;
    while value <= max_value {

        let mutator = mut_it.next().unwrap();
        for _ in 1..incr+1 {
            value_map.insert(value, (x, y));
            coord_map.insert((x, y), value);
            x += mutator.0;
            y += mutator.1;

            value = value_fn(value, &coord_map);
        }

        if idx % 2 == 0 {
            incr += 1
        }
        idx += 1;
    }

    let p1 = value_map.get(&1).unwrap();
    let p2 = value_map.get(&max_value).unwrap();

    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs())
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
    println!("Day 3 Part One = {}", spiral_memory(&input, |v, _| v + 1));

    // println!("Day 3 Part Two = {}", spiral_memory(&input, |v, coord_map| {
    //         let directions: [(i32, i32); 9] = [
    //             (0, 0), // self
    //             (0, -1), // above
    //             (1, -1), // upper right
    //             (1, 0),  // right
    //             (1, 1),  // lower right
    //             (0, 1),  // below
    //             (-1, 1),  // lower left
    //             (-1, 0),  // left
    //             (-1, -1),  // upper left
    //         ];

    //         let dirs: Vec<&u32> = directions.iter().map(|&d| {
    //             coord_map.get(&(x + d.0, y + d.1)).unwrap_or(&0)
    //         }).collect();

    //         dirs.iter().sum()
    // }));

}
