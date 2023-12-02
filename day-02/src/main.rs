fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Part 1: {}", result); // 175 too low
    println!("Hello, world!");
}


struct Set {
    red: i64,
    green: i64,
    blue: i64,
}

struct Game {
    id: i64,
    sets: Vec<Set>,
    possible: bool
}

// impl count_sets for Game {
//     fn count_sets(&self) -> i64 {
//         let mut count = 0;
//         for i in 0..self.sets.len() {
//             count += self.sets[i].red + self.sets[i].green + self.sets[i].blue;
//         }
//         return count;
//     }
// }


fn parse_game(input: &str) -> Game {
    println!("yo");
    let game_id = input.split_whitespace().nth(1).unwrap().split(":").nth(0).unwrap();
    let sets_string = input.split(":").nth(1).unwrap().split(";");
    let mut sets: Vec<Set> = Vec::new();
    for set in sets_string {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let balls = set.split(",");
        for ball in balls {
            let count = ball.split_whitespace().nth(0).unwrap();
            let kind = ball.split_whitespace().nth(1).unwrap();
            match kind {
                "red" => red = count.parse::<i64>().unwrap(),
                "green" => green = count.parse::<i64>().unwrap(),
                "blue" => blue = count.parse::<i64>().unwrap(),
                _ => println!("Error"),
            }
        }
        let set = Set {
            red,
            green,
            blue,
        };
        sets.push(set);
    }

    let mut possible = true;

    for i in 0..sets.len() {
        if sets[i].red > 12 || sets[i].green > 13 || sets[i].blue > 14 {
            print!("ææææææææææææææææææ");
            println!("{} {} {}", sets[i].red, sets[i].green, sets[i].blue);
            possible = false;
        }
    }
    
    let game = Game {
        id: game_id.parse::<i64>().unwrap(),
        sets,
        possible,
    };

    return game;
}

 
// fn count_possible_sets(game: &Game) -> i64 {

// }

fn part1(input: &str) -> i64{
    let input = 
        input
        .split("\n")
        .map(|x| parse_game(x))
        .filter(|x| x.possible == true);

    let mut id_sum = 0;
    for game in input {
        id_sum += game.id as i64;
    }

    return id_sum as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_input = include_str!("example_input.txt");
        assert_eq!(part1(test_input), 8);
    }

    #[test]
    fn test_parse_game() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let sets = vec! [
            Set {
            red: 4,
            green: 0,
            blue: 3,
        },
        Set {
            red: 1,
            green: 2,
            blue: 6,
        },
        Set {
            red: 0,
            green: 2,
            blue: 0,
        },];

        let game = Game {
            id: 1,
            sets,
            possible: true,
        };


        assert_eq!(parse_game(test_input).id, game.id);
        assert_eq!(parse_game(test_input).possible, game.possible);
        assert_eq!(parse_game(test_input).sets[0].red, game.sets[0].red);
        assert_eq!(parse_game(test_input).sets[0].green, game.sets[0].green);
        assert_eq!(parse_game(test_input).sets[0].blue, game.sets[0].blue);
        assert_eq!(parse_game(test_input).sets[1].red, game.sets[1].red);
        assert_eq!(parse_game(test_input).sets[1].green, game.sets[1].green);
        assert_eq!(parse_game(test_input).sets[1].blue, game.sets[1].blue);
        assert_eq!(parse_game(test_input).sets[2].red, game.sets[2].red);
        assert_eq!(parse_game(test_input).sets[2].green, game.sets[2].green);
        assert_eq!(parse_game(test_input).sets[2].blue, game.sets[2].blue);
    }

}