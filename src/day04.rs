use std::collections::{HashMap, HashSet};

fn part1(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(matches)
        .map(|m| if m < 2 { m } else { 2i32.pow((m-1) as u32) })
        .sum()
}

fn matches(card: &String) -> i32 {
    let count = card.split_whitespace().count();
    let uniq = card.split_whitespace().collect::<HashSet<_>>().len();
    (count - uniq) as i32
}

fn part2(input: Vec<String>) -> i32 {
    let mut wins: HashMap<usize, i32> = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        let matches = matches(line) as usize;
        for w in (i + 1)..=(i + matches) {
            *wins.entry(w).or_insert(0) += wins.get(&i).unwrap_or(&0) + 1;
        }
    }

    wins.values().sum::<i32>() + input.len() as i32
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
        assert_eq!(part1(read("day04ex.txt")), 13);
        println!("{}", part1(read("day04.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read("day04ex.txt")), 30);
        println!("{}", part2(read("day04.txt")));
    }
}