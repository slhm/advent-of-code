use regex::Regex;

fn main() {
    // https://adventofcode.com/2023/day/1

    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Part 1: {}", result);
}

fn extract_numbers(input: &str) -> String {
    input.split("\n")
            .map(|x| x.split("").filter(|x| x.parse::<i64>().is_ok()).collect::<String>())
            .collect::<Vec<_>>().join("")
}

fn first_and_last_number(input: &str) -> i64 {
    let first = input.chars().nth(0); 
    let last = input.chars().last();
    match (first, last) {
        (Some(first), Some(last)) => {
            return format!("{}{}", first, last).parse::<i64>().unwrap();
        },
        (Some(first), None) => return format!("{}{}", first, first).parse::<i64>().unwrap(),
        _ => return 0,
    }
}

fn part1(input: &str) -> i64{
    let input = 
        input
        .split("\n")
        .map(|x| extract_numbers(x))
        .map(|x| first_and_last_number(&x))
        .sum();
    return input;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = include_str!("test_input.txt");
        assert_eq!(part1(test_input), 296);
    }

    #[test]
    fn test_filter_numbers() {
        let test_input1 = "ogjeoid36oigjdf9g";
        let test_input2 = "ogjeoid36oigjdf9g11";
        let test_input3 = "4ogjeoid36oigjdf9g0";
        assert_eq!(extract_numbers(test_input1), "369");
        assert_eq!(extract_numbers(test_input2), "36911");
        assert_eq!(extract_numbers(test_input3), "43690");
    }

    #[test]
    fn test_extraction() {
        let test_input1 = "369";
        let test_input2 = "5360";
        let test_input3 = "5";
        assert_eq!(first_and_last_number(test_input1), 39);
        assert_eq!(first_and_last_number(test_input2), 50);
        assert_eq!(first_and_last_number(test_input3), 55);
    }
}