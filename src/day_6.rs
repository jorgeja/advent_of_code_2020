use std::collections::{HashSet, HashMap};

fn parse_group(g: &str) -> usize {
    let unique_chars: HashSet<char> = g.chars().filter(|c| c.is_ascii_alphabetic()).collect::<HashSet<char>>();
    unique_chars.len()
}

fn parse_common_answers(g: &str) -> usize {
    let answers: Vec<HashSet<char>> = g.lines().map(|l| l.chars().filter(|c| c.is_ascii_alphabetic()).collect::<HashSet<char>>()).collect();
    let mut common_answers = HashMap::new();
    for ans in &answers {
        for c in ans {
            *common_answers.entry(c).or_insert(0usize) += 1;
        }
    }

    common_answers.iter().filter(|(_, val)| **val == answers.len()).count()
}

#[cfg(test)]
mod test {
    use super::parse_group;
    use super::parse_common_answers;
    #[test]
    fn test () {
        let input = include_str!("example_day6.txt");
        let groups: Vec<&str> = input.split("\r\n\r\n").collect();

        let group_sums: Vec<usize> = groups.iter().map(|g| parse_group(g)).collect();
        eprintln!("Group sums {:?}", group_sums);
        let all_sum: usize = group_sums.iter().sum();
        eprintln!("Sum: {}", all_sum);
    }

    #[test]
    fn part1() {
        let input = include_str!("input_day6.txt");
        let groups: Vec<&str> = input.split("\r\n\r\n").collect();

        let sum: usize = groups.iter().map(|g| parse_group(g)).sum();
        eprintln!("Sum: {}", sum);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("example_day6_part2.txt");
        let groups: Vec<&str> = input.split("\r\n\r\n").collect();
        let sum: usize = groups.iter().map(|g| parse_common_answers(g)).sum();
        eprintln!("Group sums {:?}", sum);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day6.txt");
        let groups: Vec<&str> = input.split("\r\n\r\n").collect();
        let sum: usize = groups.iter().map(|g| parse_common_answers(g)).sum();
        eprintln!("Group sums {:?}", sum);
    }
}