use regex::{Regex, RegexBuilder};

fn main() {
    // https://adventofcode.com/2024/day/3

    let input = include_str!("input.txt");
    let input2 = include_str!("input2.txt");
    let result = part1(input);
    let result2 = part2(input2);
    
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mod_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let nums: Vec<&str> = mod_regex.find_iter(input).map(|m| m.as_str()).collect();
    let parsed_input = nums.iter().map(|x| {
        let mut split = x.split("(").nth(1).unwrap().split(")").nth(0).unwrap().split(",");
        let first = split.next().unwrap().parse::<i64>().unwrap();
        let second = split.next().unwrap().parse::<i64>().unwrap();
        return (first, second);
    }).collect();
    return parsed_input;
}

fn clean_string(input: &str) -> String {
    let remove_regex = 
        RegexBuilder::new(r"don't\(\).*?do\(\)")
        .dot_matches_new_line(true)
        .multi_line(true)
        .build()
        .unwrap();
    let mut clean_string = remove_regex.replace_all(input, "").to_string();
    let index_of_first_dirt = clean_string.find("don't()");
    if let Some(index) = index_of_first_dirt {
        clean_string = clean_string[..index].to_string();
    }
    return clean_string;
}


fn part1(input: &str) -> i64{
    let parsed_input = 
        parse_input(input);

    let result = 
        parsed_input
        .iter()
        .map(|(first, second)| {
            return  first * second
        }).sum();

    return result;
}

fn part2(input: &str) -> i64{
    let clean_string = 
        clean_string(input);
    let parsed_input =
        parse_input(clean_string.as_str());

    let result = 
        parsed_input
        .iter()
        .map(|(first, second)| {
            return  first * second
        }).sum();

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let test_input = include_str!("test_input.txt");
        assert_eq!(parse_input(test_input), vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_part1() {
        let test_input = include_str!("input.txt");
        assert_eq!(part1(test_input), 196826776);
    }

    #[test]
    fn test_part2() {
        let test_input = include_str!("input.txt");
        assert_eq!(part2(test_input), 106780429);
    }

    #[test]
    fn test_example_1() {
        let test_input = include_str!("test_input.txt");
        assert_eq!(part1(test_input), 161);
    }


    #[test]
    fn test_example_2() {
        let test_input = include_str!("test_input2.txt");
        assert_eq!(part2(test_input), 48);
    }

    #[test]
    fn test3() {
        let test_input = include_str!("test_input3.txt");
        assert_eq!(part2(test_input), 1);
    }


    #[test]
    fn test_clean_string_1() {
        let test_input = include_str!("test_input3.txt");
        assert_eq!(clean_string(test_input), "mul(1,1) ");
    }
}