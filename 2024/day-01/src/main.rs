use std::collections::{BTreeSet, HashMap};

fn main() {
    // https://adventofcode.com/2024/day/1

    let input = include_str!("input.txt");
    let result = part1(input);
    let result2 = part2(input);
    
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}

fn parse_input(input: &str) -> Vec<i64> {
    let parsed_input = 
        input
        .split("\n")
        .map(|x| x.split("   "))
        .flatten()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    return parsed_input;
}

fn part1(input: &str) -> i64{
    let parsed_input = 
        parse_input(input);

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for (i, x) in parsed_input.iter().enumerate() {
        if i % 2 == 0 {
            first_list.push(*x);
        } else {
            second_list.push(*x);
        }
    }

    first_list.sort();
    second_list.sort();

    let mut sum = 0;

    for (i, x) in first_list.iter().enumerate() {
        let second = second_list.get(i).unwrap();
        if *x > *second {
            sum += x - second;
        }else {
            sum += second - x;
        }
    }

    return sum;
}

fn part2(input: &str) -> i64{
    let parsed_input = 
        parse_input(input);

    let mut first_list = Vec::new();
    let mut second_map = HashMap::new();

    for (i, x) in parsed_input.iter().enumerate() {
        if i % 2 == 0 {
            first_list.push(*x);
        } else {
            if second_map.contains_key(x) {
                let value = second_map.get_mut(x).unwrap();
                *value += 1;
            } else {
                second_map.insert(*x, 1);
            }
        }
    }

    let mut sum = 0;

    for x in first_list {
        match second_map.get_key_value(&x){
            Some((_, value)) => {
                sum += value * x;
            },
            None => {}
        }
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_1() {
        let test_input = include_str!("test_input.txt");
        let result = parse_input(test_input);
        assert_eq!(result[0], 98415);
        assert_eq!(result[1], 86712);
        assert_eq!(result[2], 21839);
        assert_eq!(result[3], 96206);
    }
}