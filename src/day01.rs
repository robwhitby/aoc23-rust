fn calc_line(line: &String) -> i32 {
    let mut digits = line.chars().filter(char::is_ascii_digit);
    let a = digits.next().unwrap();
    let b = digits.last().unwrap_or(a);
    format!("{}{}", a, b).parse().unwrap()
}

fn replace_line(line: &String) -> String {
    let ns: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let out: String = ns
        .iter().enumerate()
        .find_map(|(i,n)| line.starts_with(n).then(|| {
            line.replacen(n, &format!("{}{}{}", n.chars().next().unwrap(), i+1, n.chars().last().unwrap()), 1)
        }))
        .unwrap_or(line.clone());

    if out.len() > 1 {
        let (head, tail) = out.split_at(1);
        head.to_string() + &replace_line(&tail.to_string())
    } else {
        out
    }
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