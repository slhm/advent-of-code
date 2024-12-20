

fn main() {
    // https://adventofcode.com/2024/day/4

    let input = include_str!("input.txt");
    let result = part1(input);
    let result2 = part2(input);
    
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}

fn search_word() -> Vec<String> {
    let search_rest = vec!["X", "M", "A", "S"].into_iter().map(String::from).collect();
    return search_rest;
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    let mut lines = vec![];
    for line in input.lines() {
        let mut letters = vec![];
        for letter in line.chars() {
            letters.push(letter.to_string());
        }
        lines.push(letters);
    }
    return lines;
}

fn check_horizontal_right(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {
    for i in 0..word.len() {
        if y + i >= input[x].len() {
            return false;
        }
        if input[x][y + i] != word[i] {
            return false;
        }
    }
    return true;
}

fn check_horizontal_left(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {

    for i in 0..word.len() {
        if y < i {
            return false;
        }
        if input[x][y - i] != word[i] {
            return false;
        }
    }
    return true;
}

fn check_vertical_down(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {
    for i in 0..word.len() {
        if x + i >= input.len() {
            return false;
        }
        if input[x + i][y] != word[i] {
            return false;
        }
    }
    return true;
}

fn check_vertical_up(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {
    for i in 0..word.len() {
        if x < i {
            return false;
        }
        if input[x - i][y] != word[i] {
            return false;
        }
    }
    return true;
}

fn check_diagonal_down_right(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {
    for i in 0..word.len() {
        if x + i >= input.len() || y + i >= input[x].len() {
            return false;
        }
        if input[x + i][y + i] != word[i] {
            return false;
        }
    }
    return true;
}

fn check_diagonal_down_left(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {
    for i in 0..word.len() {
        if x + i >= input.len() || y < i {
            return false;
        }
        if input[x + i][y - i] != word[i] {
            return false;
        }
    }
    return true;
}

fn check_diagonal_up_right(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {
    for i in 0..word.len() {
        if x < i || y + i >= input[x].len() {
            return false;
        }
        if input[x - i][y + i] != word[i] {
            return false;
        }
    }
    return true;
}

fn check_diagonal_up_left(x: usize, y: usize, word: &Vec<String>, input: &Vec<Vec<String>>) -> bool {
    for i in 0..word.len() {
        if x < i || y < i {
            return false;
        }
        if input[x - i][y - i] != word[i] {
            return false;
        }
    }
    return true;
}

fn count_xmas_at_position(x: usize, y: usize, input: &Vec<Vec<String>>) -> i64 {
    let mut count = 0;
    let word = &search_word();
    if input[x][y] == word[0] {
        if check_horizontal_right(x, y, word, input) {
            count += 1;
        }
        if check_horizontal_left(x, y, word, input) {
            count += 1;
        }
        if check_vertical_down(x, y, word, input) {
            count += 1;
        }
        if check_vertical_up(x, y, word, input) {
            count += 1;
        }
        if check_diagonal_down_right(x, y, word, input) {
            count += 1;
        }
        if check_diagonal_down_left(x, y, word, input) {
            count += 1;
        }
        if check_diagonal_up_right(x, y, word, input) {
            count += 1;
        }
        if check_diagonal_up_left(x, y, word, input) {
            count += 1;
        }
    }
    return count;
}

fn count_xmas(input: Vec<Vec<String>>) -> i64 {

    let mut count = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            count += count_xmas_at_position(x, y, &input);
        }
    }
    return count;
}

fn count_mas_at_position(x: usize, y: usize, input: &Vec<Vec<String>>) -> i64 {
    let mut count = 0;
    if input[x][y] == "A" {
        if input[x - 1][y - 1] == "M" && input[x - 1][y + 1] == "M" { // up left and up right
            if input[x + 1][y - 1] == "S" && input[x + 1][y + 1] == "S" { // down left and down right
                count += 1;
            }
        }
        if input[x - 1][y - 1] == "M" && input[x + 1][y - 1] == "M" { // up left and down left
            if input[x - 1][y + 1] == "S" && input[x + 1][y + 1] == "S" { // up right and down right
                count += 1;
            }
        }
        if input[x - 1][y + 1] == "M" && input[x + 1][y + 1] == "M" { // up right and down right
            if input[x - 1][y - 1] == "S" && input[x + 1][y - 1] == "S" { // up left and down left
                count += 1;
            }
        }
        if input[x + 1][y - 1] == "M" && input[x + 1][y + 1] == "M" { // down left and down right
            if input[x - 1][y - 1] == "S" && input[x - 1][y + 1] == "S" { // up left and up right
                count += 1;
            }
        }
    }
    return count;
}

fn count_mas(input: Vec<Vec<String>>) -> i64 {

    let mut count = 0;
    for x in 1..input.len() - 1 {
        for y in 1..input[x].len() - 1 {
            count += count_mas_at_position(x, y, &input);
        }
    }
    return count;
}

fn part1(input: &str) -> i64{
    let parsed_input = parse_input(input);

    let result = count_xmas(parsed_input);

    return result;
}


fn part2(input: &str) -> i64{
    let parsed_input = parse_input(input);

    return count_mas(parsed_input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let test_input = include_str!("input_test.txt");
        let expected = vec![
            vec!["M", "M", "M", "S", "X"],
            vec!["M", "S", "A", "M", "S"],
            vec!["S", "X", "M", "A", "S"],
        ];
        assert_eq!(parse_input(test_input), expected);
    }

    #[test]
    fn test_example_part1() {
        let test_input = include_str!("input_example.txt");
        assert_eq!(part1(test_input), 18);
    }


    #[test]
    fn test_example_part2() {
        let test_input = include_str!("input_example.txt");
        assert_eq!(part2(test_input), 9);
    }

    #[test]
    fn test_test1() {
        let test_input = include_str!("input_test.txt");
        assert_eq!(part1(test_input), 1);
    }

    #[test]
    fn test_horizontal_right() {
        let test_input = include_str!("input_test.txt");
        let input = parse_input(test_input);
        assert_eq!(check_horizontal_right(2, 1, &search_word(), &input), true);
    }

    #[test]
    fn test_horizontal_left() {
        let example = vec![
            vec!["M", "S", "A", "M", "X"].into_iter().map(String::from).collect()
        ];
        assert_eq!(check_horizontal_left(0, 4, &search_word(), &example), true);
    }

    #[test]
    fn test_up() {
        let example = vec![
            vec!["S"].into_iter().map(String::from).collect(),
            vec!["A"].into_iter().map(String::from).collect(),
            vec!["M"].into_iter().map(String::from).collect(),
            vec!["X"].into_iter().map(String::from).collect(),
        ];
        assert_eq!(check_vertical_up(3, 0, &search_word(), &example), true);
    }

    #[test]
    fn test_down() {
        let example = vec![
            vec!["X"].into_iter().map(String::from).collect(),
            vec!["M"].into_iter().map(String::from).collect(),
            vec!["A"].into_iter().map(String::from).collect(),
            vec!["S"].into_iter().map(String::from).collect(),
        ];
        assert_eq!(check_vertical_down(0, 0, &search_word(), &example), true);
    }

    #[test]
    fn test_diagonal_down_right() {
        let example = vec![
            vec!["X", "M", "A", "S"].into_iter().map(String::from).collect(),
            vec!["M", "M", "S", "X"].into_iter().map(String::from).collect(),
            vec!["A", "S", "A", "M"].into_iter().map(String::from).collect(),
            vec!["S", "X", "M", "S"].into_iter().map(String::from).collect(),
        ];
        assert_eq!(check_diagonal_down_right(0, 0, &search_word(), &example), true);
    }

    #[test]
    fn test_diagonal_down_left() {
        let example = vec![
            vec!["S", "M", "A", "X"].into_iter().map(String::from).collect(),
            vec!["M", "M", "M", "X"].into_iter().map(String::from).collect(),
            vec!["A", "A", "A", "M"].into_iter().map(String::from).collect(),
            vec!["S", "X", "M", "S"].into_iter().map(String::from).collect(),
        ];
        assert_eq!(check_diagonal_down_left(0, 3, &search_word(), &example), true);
    }

    #[test]
    fn test_diagonal_up_right() {
        let example = vec![
            vec!["S", "M", "A", "S"].into_iter().map(String::from).collect(),
            vec!["M", "M", "A", "X"].into_iter().map(String::from).collect(),
            vec!["A", "M", "A", "M"].into_iter().map(String::from).collect(),
            vec!["X", "X", "M", "S"].into_iter().map(String::from).collect(),
        ];
        assert_eq!(check_diagonal_up_right(3, 0, &search_word(), &example), true);
    }

    #[test]
    fn test_diagonal_up_left() {
        let example = vec![
            vec!["S", "M", "A", "S"].into_iter().map(String::from).collect(),
            vec!["M", "A", "A", "X"].into_iter().map(String::from).collect(),
            vec!["A", "M", "M", "M"].into_iter().map(String::from).collect(),
            vec!["X", "X", "M", "X"].into_iter().map(String::from).collect(),
        ];
        assert_eq!(check_diagonal_up_left(3, 3, &search_word(), &example), true);
    }

    #[test]
    fn test_test2() {
        let example = vec![
            vec!["S", "A", "M", "X", "S"].into_iter().map(String::from).collect(),
            vec!["X", "A", "M", "A", "S"].into_iter().map(String::from).collect(),
            vec!["X", "A", "M", "A", "S"].into_iter().map(String::from).collect(),
            vec!["S", "X", "A", "X", "X"].into_iter().map(String::from).collect(),
        ];
        assert_eq!(count_xmas(example), 4);
    }

    #[test]
    fn test_simple1() {
        let test_input = "XMAS
XMAS";
        assert_eq!(part1(test_input), 2);
    }

}