use itertools::Itertools;

fn main() {
    // https://adventofcode.com/2024/day/7

    let input = include_str!("input.txt");
    let result = part1(input);
    let result2 = part2(input);
    
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator {
    ADD,
    MUL
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator2 {
    ADD,
    MUL,
    CONCAT
}

#[derive(Debug, PartialEq)]
struct Equation {
    sum: i64,
    numbers: Vec<i64>
}

fn parse_input(input: &str) -> Vec<Equation> {
    let mut lines: Vec<Equation> = vec![];
    for line in input.lines() {
        let sum = line.split(":").nth(0).unwrap().parse::<i64>().unwrap();
        let numbers = line.split(": ").nth(1).unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        lines.push(Equation{sum, numbers});
    }
    return lines;
}

fn generate_combinations(n: usize) -> Vec<Vec<Operator>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut result = vec![];
    let smaller_combinations = generate_combinations(n - 1);

    for combination in smaller_combinations {
        let mut add_combination = combination.clone();
        add_combination.push(Operator::ADD);
        result.push(add_combination);

        let mut mul_combination = combination.clone();
        mul_combination.push(Operator::MUL);
        result.push(mul_combination);
    }

    result
}

fn part1(input: &str) -> i64{
    let parsed_input = parse_input(input);

    let mut sum: i64 = 0;
    parsed_input
        .iter()
        .filter(|equation| {
            let num_of_operators = equation.numbers.len() - 1;
            for _ in 0..num_of_operators {
                let combinations = generate_combinations(num_of_operators);
                for combination in combinations {
                    let mut result = equation.numbers[0];
                    for (j, operator) in combination.iter().enumerate() {
                        match operator {
                            Operator::ADD => result += equation.numbers[j + 1],
                            Operator::MUL => result *= equation.numbers[j + 1],
                        }
                    }
                    if result == equation.sum {
                        return true;
                    }
                }
            }
            false
        })
        .for_each(|equation| {
            sum += equation.sum;
        });

    return sum;
}

fn generate_combinations2(n: usize) -> Vec<Vec<Operator2>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut result = vec![];
    let smaller_combinations = generate_combinations2(n - 1);

    for combination in smaller_combinations {
        let mut add_combination = combination.clone();
        add_combination.push(Operator2::ADD);
        result.push(add_combination);

        let mut mul_combination = combination.clone();
        mul_combination.push(Operator2::MUL);
        result.push(mul_combination);

        let mut concat_combination = combination.clone();
        concat_combination.push(Operator2::CONCAT);
        result.push(concat_combination);
    }
    result
}

fn concat_numbers(a: i64, b: i64) -> i64 {
    let first = a.to_string();
    let second = b.to_string();
    let concat = format!("{}{}", first, second).parse::<i64>().unwrap();
    return concat;
}

fn filter_part2(equation: &Equation) -> bool {
    let num_of_operators = equation.numbers.len() - 1;
    for _ in 0..num_of_operators {
        let combinations = generate_combinations2(num_of_operators);
        for combination in combinations {
            let mut result = 0;
            let numbers = equation.numbers.clone();
            for (j, operator) in combination.iter().enumerate() {
                match operator {
                    Operator2::ADD => {
                        if j == 0 {
                            result = numbers[j] + numbers[j + 1];
                        } else {
                            result += numbers[j + 1];
                        }
                    },
                    Operator2::MUL => {
                        if j == 0 {
                            result = numbers[j] * numbers[j + 1];
                        } else {
                            result *= numbers[j + 1];
                        }
                    },
                    Operator2::CONCAT => {
                        if j == 0 {
                            result = concat_numbers(numbers[j], numbers[j + 1]);
                        }else {
                            result = concat_numbers(result, numbers[j + 1]);
                        }
                    }
                }
            }
            if result == equation.sum {
                return true;
            }
        }
    }
    false
}

fn part2(input: &str) -> i64{
    let parsed_input = parse_input(input);

    let mut sum: i64 = 0;
    parsed_input
        .iter()
        .filter(|equation| filter_part2(equation))
        .for_each(|equation| {
            sum += equation.sum;
        });

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let test_input = include_str!("input_example.txt");
        let expected = vec![
            Equation{sum: 190, numbers: vec![10, 19]},
            Equation{sum: 3267, numbers: vec![81, 40, 27]},
            Equation{sum: 83, numbers: vec![17, 5]},
            Equation{sum: 156, numbers: vec![15, 6]},
            Equation{sum: 7290, numbers: vec![6, 8, 6, 15]},
            Equation{sum: 161011, numbers: vec![16, 10, 13]},
            Equation{sum: 192, numbers: vec![17, 8, 14]},
            Equation{sum: 21037, numbers: vec![9, 7, 18, 13]},
            Equation{sum: 292, numbers: vec![11, 6, 16, 20]},
        ];
        assert_eq!(parse_input(test_input), expected);
    }

    #[test]
    fn test_concat_numbers() {
        assert_eq!(concat_numbers(1, 2), 12);
        assert_eq!(concat_numbers(12, 3), 123);
        assert_eq!(concat_numbers(123, 4), 1234);
    }

    #[test]
    fn test_filter_part2() {
        let test_input = include_str!("input_example.txt");
        let parsed_input = parse_input(test_input);
        assert_eq!(filter_part2(&parsed_input[0]), true);
        assert_eq!(filter_part2(&parsed_input[1]), true);
        assert_eq!(filter_part2(&parsed_input[2]), false);
        assert_eq!(filter_part2(&parsed_input[3]), true);
        assert_eq!(filter_part2(&parsed_input[4]), true);
        assert_eq!(filter_part2(&parsed_input[5]), false);
        assert_eq!(filter_part2(&parsed_input[6]), true);
        assert_eq!(filter_part2(&parsed_input[7]), false);
        assert_eq!(filter_part2(&parsed_input[8]), true);
    }

    #[test]
    fn filter2_test() {
        let eq = Equation{sum: 195, numbers: vec![19, 5]};
        assert_eq!(filter_part2(&eq), true);
    }

    #[test]
    fn filter2_test2() {
        let eq = Equation{sum: 7290, numbers: vec![6, 8, 6, 15]};
        assert_eq!(filter_part2(&eq), true);
    }

    #[test]
    fn filter2_test3() {
        let eq = Equation{sum: 105, numbers: vec![10, 10, 5]};
        assert_eq!(filter_part2(&eq), true);
    }

    #[test]
    fn test_part2_mini() {
        let text = include_str!("input_test2.txt");
        assert_eq!(part2(text), 300);
    }

    #[test]
    fn test_generate_combinations2() {
        let expected1 = vec![
            vec![Operator2::ADD],
            vec![Operator2::MUL],
            vec![Operator2::CONCAT]
        ];

        let expected2 = vec![
            vec![Operator2::ADD, Operator2::ADD],
            vec![Operator2::ADD, Operator2::MUL],
            vec![Operator2::ADD, Operator2::CONCAT],
            vec![Operator2::MUL, Operator2::ADD],
            vec![Operator2::MUL, Operator2::MUL],
            vec![Operator2::MUL, Operator2::CONCAT],
            vec![Operator2::CONCAT, Operator2::ADD],
            vec![Operator2::CONCAT, Operator2::MUL],
            vec![Operator2::CONCAT, Operator2::CONCAT]
        ];
        
        assert_eq!(generate_combinations2(0), vec![vec![]]);
        assert_eq!(generate_combinations2(1), expected1);
        assert_eq!(generate_combinations2(2), expected2);
    }
    #[test]
    fn test_example1() {
        let test_input = include_str!("input_example.txt");
        assert_eq!(part1(test_input), 3749);
    }

    #[test]
    fn test_example2() {
        let test_input = include_str!("input_example.txt");
        assert_eq!(part2(test_input), 11387);
    }


    #[test]
    fn test_part1() {
        let test_input = include_str!("input.txt");
        assert_eq!(part1(test_input), 5702958180383);
    }

    // dont actually run this. just run it and compare outputs instead
    // #[test]
    // fn test_part2() {
    //     let test_input = include_str!("input.txt");
    //     assert_eq!(part2(test_input), 92612386119138);
    // }

}