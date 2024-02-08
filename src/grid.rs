use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub value: char,
}

pub struct Grid(pub Vec<String>);

impl Grid {
    pub fn get(&self, x: i32, y: i32) -> Option<Point> {
        self.0
            .get(y as usize)
            .and_then(|r| r.chars().nth(x as usize))
            .map(|value| Point { x, y, value })
    }

    pub fn neighbours(&self, p: &Point) -> HashSet<Point> {
        ((p.x - 1)..=(p.x + 1))
            .flat_map(|x| ((p.y - 1)..=(p.y + 1)).map(move |y| (x, y)))
            .filter(|&n| n != (p.x, p.y))
            .flat_map(|(x,y)| self.get(x,y))
            .collect()
    }

    pub fn iter(&self) -> GridIter<'_> {
        GridIter {
            grid: self,
            current: None,
        }
    }
}

pub struct GridIter<'a> {
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