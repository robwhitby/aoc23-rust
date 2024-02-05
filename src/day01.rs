fn calc_line(line: &String) -> i32 {
    let mut digits = line.chars().filter(char::is_ascii_digit);
    let a = digits.next().unwrap();
    let b = digits.last().unwrap_or(a);
    format!("{}{}", a, b).parse().unwrap()
}

fn replace_line(line: &String) -> String {
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut out = line.clone();
    for (i,n) in nums.iter().enumerate() {
        out = out.replace(n, &format!("{}{}{}", n, i+1, n))
    }
    out
}

fn part1(input: Vec<String>) -> i32 {
    input.iter().map(calc_line).sum()
}

fn part2(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(replace_line)
        .map(|l| calc_line(&l))
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
        assert_eq!(part1(read("day01ex.txt")), 142);
        println!("{}", part1(read("day01.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read("day01ex2.txt")), 281);
        println!("{}", part2(read("day01.txt")));
    }
}