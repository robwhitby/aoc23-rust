use std::collections::HashSet;
use crate::grid::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Number(Vec<Point>);

impl Number {
    fn neighbours(&self, grid: &Grid) -> HashSet<Point> {
        self.0.iter()
            .flat_map(|p| grid.neighbours(p))
            .filter(|p| !self.0.contains(p))
            .collect()
    }

    fn value(&self) -> i32 {
        self.0.iter()
            .map(|p| p.value)
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn contains(&self, p: &Point) -> bool {
        self.0.contains(p)
    }
}

fn parse_numbers(grid: &Grid) -> Vec<Number> {
    let mut nums: Vec<Number> = vec![];
    let mut current: Vec<Point> = vec![];
    for p in grid.iter() {
        if p.value.is_ascii_digit() && p.y == current.get(0).map(|n| n.y).unwrap_or(p.y) {
            current.push(p)
        }
        else if current.len() > 0 {
            nums.push(Number(current.clone()));
            current.clear();
        }
    }
    nums
}

fn part1(input: Vec<String>) -> i32 {
    let grid = Grid(input);
    parse_numbers(&grid)
        .iter()
        .filter(|n| n.neighbours(&grid).iter().any(|p| !p.value.is_ascii_digit() && p.value != '.'))
        .map(|n| n.value())
        .sum()
}

fn part2(input: Vec<String>) -> i32 {
    let grid = Grid(input);
    let numbers = parse_numbers(&grid);

    grid
        .iter()
        .filter(|p| p.value == '*')
        .filter_map(|s| {
            let ns = grid.neighbours(&s);
            let nums: Vec<&Number> = numbers.iter().filter(|num| ns.iter().any(|n| num.contains(n))).collect();
            (nums.len() == 2).then(|| {nums[0].value() * nums[1].value()})
        })
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
        assert_eq!(part1(read("day03ex.txt")), 4361);
        println!("{}", part1(read("day03.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read("day03ex.txt")), 467835);
        println!("{}", part2(read("day03.txt")));
    }
}
