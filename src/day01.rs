fn calc_line(line: &String) -> i32 {
    let digits: Vec<char> = line
        .chars()
        .filter(char::is_ascii_digit)
        .collect();

    let s = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    s.parse::<i32>().unwrap()
}

fn replace_line(line: &String) -> String {
    let s = match line.as_str() {
        l if line.starts_with("one") => l.replacen("one", "o1e", 1),
        l if line.starts_with("two") => l.replacen("two", "t2o", 1),
        l if line.starts_with("three") => l.replacen("three", "t3e", 1),
        l if line.starts_with("four") => l.replacen("four", "f4r", 1),
        l if line.starts_with("five") => l.replacen("five", "f5e", 1),
        l if line.starts_with("six") => l.replacen("six", "s6x", 1),
        l if line.starts_with("seven") => l.replacen("seven", "s7n", 1),
        l if line.starts_with("eight") => l.replacen("eight", "e8t", 1),
        l if line.starts_with("nine") => l.replacen("nine", "n9e", 1),
        l => l.to_string()
    };

    let head: String = s.chars().take(1).collect();
    let tail: String = s.chars().skip(1).collect();

    if tail.len() > 0 {
        head + &replace_line(&tail)
    } else {
        head
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