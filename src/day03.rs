#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    value: char,
}

struct Grid(Vec<String>);

impl Grid {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.get(x, y).is_some()
    }

    fn get(&self, x: i32, y: i32) -> Option<Point> {
        self.0
            .get(y as usize)
            .and_then(|r| r.chars().nth(x as usize))
            .map(|value| Point { x, y, value })
    }

    fn neighbours(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        ((x - 1)..=(x + 1))
            .flat_map(|x| ((y - 1)..=(y + 1)).map(move |y| (x, y)))
            .filter(|&n| n != (x, y))
            .collect()
    }

    fn iter(&self) -> GridIter<'_> {
        GridIter {
            grid: self,
            current: None,
        }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    current: Option<Point>,
}

impl Iterator for GridIter<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.current.unwrap_or(Point {
            x: -1,
            y: 0,
            value: ' ',
        });
        self.current = self.grid.get(p.x + 1, p.y).or(self.grid.get(0, p.y + 1));
        self.current
    }
}

fn part1(input: Vec<String>) -> i32 {
    Grid(input)
        .iter()
        .filter(|p| p.value.is_ascii_digit())
        .count() as i32
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
