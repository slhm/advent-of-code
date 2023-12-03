use std::error::Error;


type Schematic = Vec<Vec<char>>;

trait SchematicAccess {
    fn get(&self, y: usize, x: usize) -> char;
    fn push(&mut self, value: Vec<char>);
    fn len_x(&self) -> usize;
    fn len_y(&self) -> usize;
}
impl SchematicAccess for Schematic {
    fn get(&self, y: usize, x: usize) -> char {
        return self[y][x];
    }

    fn push(&mut self, value: Vec<char>) {
        self.push(value);
    }

    fn len_x(&self) -> usize {
        return self.len();
    }
    fn len_y(&self) -> usize {
        return self[0].len();
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Part 1: {}", result); // 203424 too low, 550209 too high, 416067 wrong
}

fn num_digits(a: i64) -> u32 {
    return (a.abs() as f64 + 0.1).log10().ceil() as u32
}

fn check_part_number(schematic: Schematic, i: usize, j: usize) -> Result<(), String> {
    if i >= schematic.len_x() || j >= schematic.len_y() {
        return Err("error".to_string());
    }
    if schematic.get(i, j) != '.' && !schematic.get(i, j).is_numeric() {return Ok(())}
    return Err("error".to_string());
}

fn is_part_number_member(schematic: Schematic, y: usize, x: usize) -> bool {
    if y > schematic.len() || x > schematic.len_y() {
        return false;
    }
    if schematic.get(y, x).is_numeric() {return true}
    return false;
}

fn check_surrounding_indices(schematic: Schematic, num_length: i64, y: usize, x: usize) -> Result<(), String> {
    for i in -1..1 {
        for j in -1..num_length+1 {
            // println!("i: {}, j: {}", i, j);
            if i == 0 && j == 0  {continue;}
            if check_part_number(schematic.clone(), (y as i64 + i) as usize, (x as i64 + j) as usize).is_ok() {
                // println!("ok!");
                return Ok(());
            }
        }
    }
    return Err("error".to_string());
}

fn find_part_numbers(schematic: Schematic) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    'columnn: for column in 0..schematic.len() {
        'row: for row in 0..schematic[column].len() {
            if schematic[column][row].is_numeric() {
                // println!("row: {}, column: {}", row, column);
                if is_part_number_member(schematic.clone(), column, row - 1) {continue 'row;}
                // println!("hei1");
                
                let mut part_number: Vec<char> = vec![schematic[column][row]];
                let mut num_length = 1;
                if row < schematic[column].len() - 1 {if schematic[column][row+1].is_numeric() { part_number.push(schematic[column][row+1]); num_length += 1;}}
                if row < schematic[column].len() - 2 {if schematic[column][row+2].is_numeric() { part_number.push(schematic[column][row+2]); num_length += 1;}}
                
                let check_indices = check_surrounding_indices(schematic.clone(), num_length, column, row);
                match check_indices {
                    Ok(_) => {
                        let mut part_number_string = String::new();
                        for character in part_number {
                            part_number_string.push(character);
                        }
                        result.push(part_number_string.parse::<i64>().unwrap());
                    },
                    Err(_) => ()
                }
            }
        }
    }
    // println!("{:?}", result);
    return result;
}

fn parse_schematic(input: &str) -> Result<Schematic, String> {
    let mut result: Schematic = Vec::new();
    for line in input.lines() {
        let mut sides: Vec<char> = Vec::new();
        for character in line.chars() {
            sides.push(character);
        }
        result.push(sides);
    }
    return Ok(result);
}

fn part1(input: &str) -> i64 {
    let result = 
         parse_schematic(input)
         .map(|schematic| find_part_numbers(schematic))
         .map(|part_numbers| part_numbers.iter().sum());
         
    return result.unwrap();
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part1() {
        let test_input = include_str!("example_input.txt");
        assert_eq!(part1(test_input), 4361);
    }

    #[test]
    fn test_parsing(){
        let test_input: &str = "..4\n.*.\n.4.";
        let expected: Result<Schematic,String> = Ok(vec![
            vec!['.', '.', '4'],
            vec!['.', '*', '.'],
            vec!['.', '4', '.']
        ]);
        assert_eq!(parse_schematic(test_input), expected);
    }

    #[test]
    fn test_part_number_check(){
        let input: Result<Schematic, String> = Ok(vec![
            vec!['.', '.', '4', '5'],
            vec!['.', '*', '.', '.'],
            vec!['.', '4', '.', '.'],
            vec!['.', '.', '#', '?']
        ]);
        assert_eq!(check_part_number(input.clone().unwrap(), 0, 3), Err("error".to_string()));
        assert_eq!(check_part_number(input.clone().unwrap(), 1, 1), Ok(()));
        assert_eq!(check_part_number(input.clone().unwrap(), 3, 2), Ok(()));
        assert_eq!(check_part_number(input.clone().unwrap(), 3, 3), Ok(()));
        assert_eq!(check_part_number(input.clone().unwrap(), 0, 5), Err("error".to_string()));
        assert_eq!(check_part_number(input.clone().unwrap(), 2, 1), Err("error".to_string()));
    }

    #[test]
    fn test_part1_sample(){
        let input: &str = "..4\n.*.\n.4.";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part1_sample2(){
        let input: &str = "..45\n.*..\n.4..\n..#?";
        assert_eq!(part1(input), 49);
    }

    #[test]
    fn test_part1_sample3(){
        let input: &str = ".245\n*...\n.4..\n..#?";
        assert_eq!(part1(input), 249);
    }

    #[test]
    fn test_part1_sample4(){
        let input: &str = "245.\n.*..\n14..\n.#1?";
        assert_eq!(part1(input), 260);
    }

    #[test]
    fn test_check_surrounding_indices(){
        let input: Result<Vec<Vec<char>>,String> = Ok(vec![
            vec!['.', '.', '4', '.'],
            vec!['.', '*', '.', '.'],
            vec!['.', '4', '.', '.'],
            vec!['.', '.', '.', '.']
        ]);
        assert_eq!(check_surrounding_indices(input.clone().unwrap(), 1, 1, 1), Err("error".to_string()), "is not a number and should return error");
        assert_eq!(check_surrounding_indices(input.clone().unwrap(), 1, 0, 2), Ok(()), "is a number, should hit * at -1,+1 and return ok");
    }
    

    #[test]
    fn test_test(){
        assert_eq!(num_digits(623), 3);
        assert_eq!(num_digits(62300), 5);
        assert_eq!(num_digits(6), 1);
    }
}