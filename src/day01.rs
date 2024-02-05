fn calc_line(line: &String) -> i32 {
    let digits: Vec<char> = line
        .chars()
        .filter(char::is_ascii_digit)
        .collect();

    let s = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    s.parse::<i32>().unwrap()
}

fn replace_line(line: &String) -> String {
    let mut out = line.clone();
    let ns: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for (i, n) in ns.iter().enumerate() {
        if out.starts_with(n) {
            out = out.replacen(n, &format!("{}{}{}", n.chars().nth(0).unwrap(), i+1, n.chars().last().unwrap()), 1);
            break
        }
    }

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
    let input2 = input.iter().map(replace_line).collect();
    part1(input2)
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