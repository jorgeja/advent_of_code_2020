
fn parse_input(input: &str) -> Vec<i32> {
    let mut parsed_input: Vec<i32> = input.lines().filter_map(|l| l.parse::<i32>().ok()).collect();
    parsed_input.sort();
    parsed_input
}   

use std::collections::HashMap;
fn find_differences(adapters: &[i32]) -> HashMap<i32, i32> {
    let mut differences = HashMap::new();
    adapters.windows(2).for_each(|s|{
            let diff = s[0]-s[1];
            *differences.entry(diff.abs()).or_insert(0) += 1;
        });
    differences   
}

fn find_possible_paths(value: i32, adapters: &[i32], cache: &mut HashMap<i32, usize>) -> usize {
    if let Some(count) = cache.get(&value) {
        eprintln!("Hit cache!");
        return *count;
    }

    let mut num_possible_paths = 0;
    for (index, adapter) in adapters.iter().enumerate() {
        if *adapter - value <= 3 {
            if index + 1 == adapters.len() {
                num_possible_paths += 1;
            } else {
                num_possible_paths += find_possible_paths(*adapter, &adapters[index+1..], cache)
            }
        } else {
            break;
        }
    }
    *cache.entry(value).or_insert(num_possible_paths)       
}

fn find_paths_faster(adapters: &[i32]) -> u128 {
    let mut num = 1;
    adapters.iter().enumerate().for_each(|(i,a)|{
        let mut next_possible_paths = 0;
        for next_adapter in adapters[i..].iter() {
            if *next_adapter - *a <= 3 {
                next_possible_paths += 1
            } else {
                break;
            }
        }
        num += next_possible_paths;
    });
    num * adapters.len() as u128 
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{find_differences, find_paths_faster, find_possible_paths, parse_input};
    
    #[test]
    fn part1_example() {
        let input = include_str!("example_day10.txt");
        let mut adapters = parse_input(input);        
        adapters.insert(0, 0);
        adapters.push(adapters.last().unwrap() + 3);
        let diffs = find_differences(&adapters);
        eprintln!("{:?}", diffs);
    }

    #[test]
    fn part1() {
        let input = include_str!("input_day10.txt");
        let mut adapters = parse_input(input);        
        adapters.insert(0, 0);
        adapters.push(adapters.last().unwrap() + 3);
        let diffs = find_differences(&adapters);
        eprintln!("{:?}", diffs);
        let ones = diffs.get(&1).unwrap_or(&0);
        let threes = diffs.get(&3).unwrap_or(&0);
        eprintln!("Result {}", ones*threes);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("example_day10.txt");
        let mut adapters = parse_input(input);        
        //adapters.insert(0, 0);   
        adapters.push(adapters.last().unwrap() + 3);
        let num_paths = find_possible_paths(0, &adapters, &mut HashMap::new());
        eprintln!("Num Paths {}", num_paths);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day10.txt");
        let mut adapters = parse_input(input);     
        //adapters.insert(0, 0);   
        adapters.push(adapters.last().unwrap() + 3);
        let num_paths = find_possible_paths(0, &adapters, &mut HashMap::new());
        eprintln!("Num Paths {}", num_paths);
    }
}