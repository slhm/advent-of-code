use core::num;
use std::fmt::format;



fn main() {
    // https://adventofcode.com/2023/day/1

    let input = include_str!("input.txt");
    let result = part1(input);
    let result2 = part2(input);
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2); // under 54888 over 
}

fn extract_numbers_part1(input: &str) -> String {
    input.split("\n")
        .map(|x| x.split("").filter(|x| (x.parse::<i64>().is_ok())).collect::<String>())
        .collect::<Vec<_>>().join("")
}

fn convert_string_numbers(input: &str) -> String {
    const NUMBERS_AS_STRINGS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    const START_OF_NUMBERS: [char; 7] = ['z', 'o', 't', 'f', 's', 'e', 'n'];
    const NUMBERS_AS_STRINGS_REV: [&str; 10] = ["ero", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
    const START_OF_NUMBERS_REV: [char; 6] = ['o', 'e', 'r', 'x', 'n', 't'];
    let mut converted_numbers = input.to_string();
    let mut first_number = String::new();
    let mut last_number = String::new();
    'outer: for i in 0..converted_numbers.len() {
        if converted_numbers.chars().nth(i).unwrap().is_ascii_digit() {
            first_number = converted_numbers.chars().nth(i).unwrap().to_string();
            break;
        }
        else {
            if START_OF_NUMBERS.contains(&converted_numbers.chars().nth(i).unwrap()) {
                '_inner: for j in 0..NUMBERS_AS_STRINGS.len() { 
                    if converted_numbers.split_at(i).1.starts_with(NUMBERS_AS_STRINGS[j]) { // bug: denne sjekker alltid hele strengen
                        first_number = j.to_string();
                        converted_numbers = converted_numbers.split_at(i + NUMBERS_AS_STRINGS[j].len() - 1).1.to_string();
                        break 'outer;
                    }
                }
            }
        }
    }
    let reversed_string = converted_numbers.chars().rev().collect::<String>();
    'outer: for i in 0..reversed_string.len() {
        if reversed_string.chars().nth(i).unwrap().is_ascii_digit() {
            last_number = reversed_string.chars().nth(i).unwrap().to_string();
            break;
        }
        else {
            if START_OF_NUMBERS_REV.contains(&reversed_string.chars().nth(i).unwrap()) {
                '_inner: for j in 0..NUMBERS_AS_STRINGS_REV.len() {
                    if reversed_string.starts_with(NUMBERS_AS_STRINGS_REV[j]) {
                        last_number = j.to_string();
                        break 'outer;
                    }
                }
            }
        }
    }
    format!("{}{}", first_number, last_number)
}

fn convert_string_numbers_backup(input: &str) -> String {
    let numbers_as_strings: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut converted_numbers = input.to_string();
    for i in 0..numbers_as_strings.len() {
        let tmp = converted_numbers.replace(numbers_as_strings[i], &i.to_string());
        converted_numbers = tmp;
    }
    converted_numbers
}

fn extract_numbers_part2(input: &str) -> String {
    input.split("\n")
        .map(|x| convert_string_numbers(&x))
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
        .map(|x| extract_numbers_part1(x))
        .map(|x| first_and_last_number(&x))
        .sum();
    return input;
}

fn part2(input: &str) -> i64{
    let input = 
        input
        .split("\n")
        .map(|x| extract_numbers_part2(x))
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
        assert_eq!(part1(test_input), 325);
    }

    #[test]
    fn test_part2() {
        let test_input = include_str!("test_input.txt");
        assert_eq!(part2(test_input), 324);
    }

    #[test]
    fn test_filter_numbers_part1() {
        let test_input1 = "ogjeoid36oigjdf9g";
        let test_input2 = "ogjeoid36oigjdf9g11";
        let test_input3 = "4ogjeoid36oigjdf9g0";
        assert_eq!(extract_numbers_part1(test_input1), "369");
        assert_eq!(extract_numbers_part1(test_input2), "36911");
        assert_eq!(extract_numbers_part1(test_input3), "43690");
    }

    #[test]
    fn test_filter_numbers_part2() {
        let test_input1 = "ogtwojeoid3zero6oigjdf9gfivefive";
        assert_eq!(convert_string_numbers(&test_input1), "25");
    }

    #[test]
    fn test_filter_numbers_part2222() {
        let test_input1 = "ogtwojeoid3zero6oigjdf9gfivefive";
        assert_eq!(extract_numbers_part2(&test_input1), "25");
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

    #[test]
    fn test_example_part2() {
        let test_input1 = "two1nine\n
        eightwothree\n
        abcone2threexyz\n
        xtwone3four\n
        4nineeightseven2\n
        zoneight234\n
        7pqrstsixteen";
        assert_eq!(part2(test_input1), 281);
    }

    #[test]
    fn test_bug1() {
        let test_input1 = "zoneight234";
        assert_eq!(convert_string_numbers(&test_input1), "14");
    }

    #[test]
    fn test_bug2() {
        let test_input1 = "zoneight";
        assert_eq!(convert_string_numbers(&test_input1), "11");
    }
}