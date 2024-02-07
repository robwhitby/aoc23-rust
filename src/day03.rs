use crate::grid::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Number{
    p: Point,
    value: i32,
}

impl Number {
    fn neighbours(&self, grid: &Grid) -> Vec<Point> {
        ((self.p.x - 1)..=(self.p.x + self.value.to_string().len() as i32))
            .flat_map(|x| ((self.p.y - 1)..=(self.p.y + 1)).map(move |y| (x, y)))
            .flat_map(|(x,y)| grid.get(x,y))
            .collect()
    }
}

fn parse_numbers(grid: &Grid) -> Vec<Number> {
    let mut nums: Vec<Number> = vec![];
    let mut start: Option<Point> = None;
    let mut acc = "".to_string();
    for p in grid.iter() {
        if p.value.is_ascii_digit() && p.y == start.map(|p| p.y).unwrap_or(p.y) {
            start = start.or(Some(p));
            acc.push(p.value)
        }
        else if start.is_some() {
            nums.push(Number{p: start.unwrap(), value: acc.parse().unwrap() });
            start = None;
            acc = "".to_string()
        }
    }
    nums
}

fn part1(input: Vec<String>) -> i32 {
    let grid = Grid(input);
    parse_numbers(&grid)
        .iter()
        .filter(|n| n.neighbours(&grid).iter().any(|p| !p.value.is_ascii_digit() && p.value != '.'))
        .map(|n| n.value)
        .sum()
}

fn part2(input: Vec<String>) -> i32 {
    1
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
