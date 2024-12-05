use std::collections::HashMap;

fn main() {
    // https://adventofcode.com/2024/day/2

    let input = include_str!("input.txt");
    let result = part1(input);
    let result2 = part2(input);
    
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let parsed_input = 
        input
        .split("\n")
        .map(|line| {
            line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    return parsed_input;
}

fn check_rules(mut line: Vec<i64>) -> bool {
    let mut inc_dec = 0;
    let mut i = 0;
    while i <= line.len() {
        if i == line.len()-1 {
            return true;
        }
        match inc_dec {
            0 => {
                if &line[i] < &line[i+1] { // the line is increasing
                    inc_dec = 1;
                } else if &line[i] > &line[i+1] { // the line is decreasing
                    inc_dec = -1;
                }
                else {
                    println!("line unsafe1 {}", line.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
                    return false;
                }
            },
            1 => {
                if &line[i] > &line[i+1] { // should be increasing but is decreasing
                    println!("line unsafe2 {}", line.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
                    return false;
                }
            },
            -1 => {
                if &line[i] < &line[i+1] { // should be decreasing but is increasing
                    println!("line unsafe3 {}", line.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
                    return false;
                }
            },
            _ => {}
        }
        if (&line[i] - &line[i+1]).abs() > 3 || &line[i] - &line[i+1].abs() == 0 { // jump over 3 or 0
            println!("line unsafe4 {}", line.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
            return false;
        }
        i += 1;
    }
    return true;
}

fn part1(input: &str) -> i64{
    let parsed_input = 
        parse_input(input);

    let mut sum = 0;
    for line in parsed_input {
        if check_rules(line.clone()) {
            sum += 1;
        }
    }
    return sum;
}

fn generate_versions(vec: Vec<i64>) -> Vec<Vec<i64>> {
    let mut result = Vec::new();
    for i in 0..vec.len() {
        let mut new_vec = vec.clone();
        new_vec.remove(i);
        result.push(new_vec);
    }
    result
}

fn part2(input: &str) -> i64{
    let parsed_input = 
        parse_input(input);

    let mut sum = 0;
    for line in parsed_input {
        let versions = generate_versions(line.clone());
        for version in versions {
            if check_rules(version.clone()) {
                sum += 1;
                break;
            }
        }
    }
    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(parse_input("7 6 4 2 1\n1 2 7 8 9"), vec![[7, 6, 4, 2, 1], [1, 2, 7, 8, 9]]);
    }

    #[test]
    fn test_input_part1() {
        let test_input = include_str!("test_input.txt");
        assert_eq!(part1(test_input), 2);
    }

    #[test]
    fn test_part1() {
        let test_input = include_str!("input.txt");
        assert_eq!(part1(test_input), 257);
    }


    #[test]
    fn test_part2() {
        let test_input = include_str!("input.txt");
        assert_eq!(part2(test_input), 328);
    }

    #[test]
    fn test_input_part2() {
        let test_input = include_str!("test_input.txt");
        assert_eq!(part2(test_input), 7);
    }
    #[test]
    fn testtest() {
        let test_input = include_str!("testtest.txt");
        assert_eq!(part2(test_input), 1);
    }
}