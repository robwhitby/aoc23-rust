use regex::{Regex};

#[derive(Debug)]
struct Game {
    id: i32,
    r: i32,
    g: i32,
    b: i32,
}

fn parse_game(id: i32, line: &String) -> Game {
    let colour = |c: &str| -> i32 {
        Regex::new(&format!(r"(\d+) {}", c))
            .unwrap()
            .captures_iter(line)
            .map(|caps| caps.get(1).unwrap().as_str().parse::<i32>().unwrap())
            .max()
            .unwrap()
    };
    Game{ id: id, r: colour("red"), g: colour("green"), b: colour("blue") }
}

fn part1(input: Vec<String>) -> i32 {
    input
        .iter().enumerate()
        .map(|(idx, line)| parse_game(idx as i32 + 1, line))
        .filter(|g| g.r <= 12 && g.g <= 13 && g.b <= 14)
        .map(|g| g.id)
        .sum()

}

fn part2(input: Vec<String>) -> i32 {
    input
        .iter().enumerate()
        .map(|(idx, line)| parse_game(idx as i32 + 1, line))
        .map(|g| g.r * g.g * g.b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(filename: &str) -> Vec<String> {
        std::fs::read_to_string("inputs/".to_string() + filename)
            .unwrap()
            .lines()
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(read("day02ex.txt")), 8);
        println!("{}", part1(read("day02.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read("day02ex.txt")), 2286);
        println!("{}", part2(read("day02.txt")));
    }
}