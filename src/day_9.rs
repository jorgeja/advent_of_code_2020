

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().filter_map(|l| l.parse::<i64>().ok()).collect()
}

fn find_invalid_entry(stream: &[i64], preamble: usize) -> Option<(usize, i64)> {        
    for (index, val) in stream[preamble..].iter().enumerate() {        
        //eprintln!("{}",val);
        let real_index = preamble + index;       
        let valid = stream[index..real_index].iter().any(|v1| stream[index..real_index-1].iter().any(|v2|{*v1 != *v2 && *v1 + *v2 == *val}));
        if !valid {
            return Some((real_index, *val));
        }
    }
    None
}

fn find_contigious_min_max(value: i64, stream: &[i64]) -> Option<(i64, i64)> {
    for range in 0..stream.len() {
        for sub_section in 0..(stream.len() - range) {
            let range = &stream[sub_section..sub_section+range];
            let sum:i64 = range.iter().sum();
            if sum == value {
                let min = *range.iter().min().unwrap_or(&-1);
                let max= *range.iter().max().unwrap_or(&-1);
                return Some((min, max))
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::{find_contigious_min_max, parse_input, find_invalid_entry};

    #[test]
    fn part1_example() {
        let input = include_str!("example_day9.txt");
        let stream = parse_input(input);
        let result = find_invalid_entry(&stream, 5);        
        eprintln!("First invalid number {:?}", result);
    }

    #[test]
    fn part1() {
        let input = include_str!("input_day9.txt");
        let stream = parse_input(input);
        let result = find_invalid_entry(&stream, 25);
        eprintln!("First invalid number {:?}", result);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("example_day9.txt");
        let stream = parse_input(input);
        let result = find_invalid_entry(&stream, 5);
        eprintln!("First invalid number {:?}", result);
        if let Some((index, value)) = result {
            let min_max = find_contigious_min_max(value, &stream[..index]);
            if let Some((min, max)) = min_max {
                eprintln!("Result {}<{}", min, max);
            } else {
                eprintln!("Could not find a result :(");
            }
            
        }
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day9.txt");
        let stream = parse_input(input);
        let result = find_invalid_entry(&stream, 25);
        eprintln!("First invalid number {:?}", result);
        if let Some((index, value)) = result {
            let min_max = find_contigious_min_max(value, &stream[..index]);
            if let Some((min, max)) = min_max {
                eprintln!("Result {} + {} = {}", min, max, min + max);
            } else {
                eprintln!("Could not find a result :(");
            }
            
        }
    }
}