use std::ops::Range;
use itertools::Itertools;

struct Data{
    seeds: Vec<i64>,
    converters: Vec<Converter>
}

impl Data {
    fn convert(&self, i: &i64) -> i64 {
        self.converters.iter().fold(i.to_owned(), |acc, i| i.convert(&acc))
    }

    fn reverse(&self, i: &i64) -> i64 {
        self.converters.iter().rev().fold(i.to_owned(), |acc, i| i.reverse(&acc))
    }
}

struct Converter(Vec<Offset>);

impl Converter {
    fn convert(&self, i: &i64) -> i64 {
        self.0.iter().find(|o| o.range.contains(&i)).map(|o| o.delta).unwrap_or(0) + i
    }

    fn reverse(&self, i: &i64) -> i64 {
        self.0.iter().find(|o| o.range.contains(&(i - o.delta))).map(|o| i - o.delta).unwrap_or(i.to_owned())
    }
}

struct Offset{
    range: Range<i64>,
    delta: i64
}

impl Offset {
    fn from(s: String) -> Offset {
        let ns: Vec<i64> = s.split_whitespace().flat_map(|i| i.parse::<i64>()).collect();
        Offset{ range: ns[1]..(ns[1] + ns[2]), delta: ns[0] - ns[1] }
    }
}

fn parse_input(input: Vec<String>) -> Data {
    let seeds: Vec<i64> = input
        .first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(|v| v.parse::<i64>())
        .collect();

    let mut converters: Vec<Converter> = Vec::new();

    for (ok, it) in &input.into_iter().group_by(|line| line.starts_with(|c:char| c.is_ascii_digit())) {
        if ok {
            let offsets = it.map(Offset::from).collect();
            converters.push(Converter(offsets));
        }
    }

    Data{ seeds, converters }
}

fn part1(input: Vec<String>) -> i64 {
    let data = parse_input(input);
    data.seeds
        .iter()
        .map(|seed| data.convert(seed))
        .min()
        .unwrap()
}

fn part2(input: Vec<String>) -> i64 {
    let data = parse_input(input);
    let seeds: Vec<Range<i64>> = data.seeds.iter()
        .tuples()
        .map(|(a,b)| a.to_owned()..a+b)
        .collect();

    (1..)
        .find(|i| seeds.iter().any(|r| r.contains(&data.reverse(i))))
        .unwrap()

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
        assert_eq!(part1(read("day05ex.txt")), 35);
        println!("{}", part1(read("day05.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read("day05ex.txt")), 46);
        println!("{}", part2(read("day05.txt")));
    }
}