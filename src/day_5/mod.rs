#[derive(Debug)]
struct BoardingPass {
    row: u8,
    col: u8,
    id: i32,
}

fn parse_line(s: &str) -> BoardingPass {
    let mut max_row = 128;
    let mut min_row = 0;
    let mut max_col = 8;
    let mut min_col = 0;
    for c in s.chars() {
        match c {
            'F' => {
                max_row = min_row + ((max_row - min_row) / 2);                
            },
            'B' => {
                min_row = min_row + (max_row - min_row) / 2;
            },
            'L' => {
                max_col = min_col + ((max_col - min_col) / 2);                
            },
            'R' => {
                min_col = min_col + (max_col - min_col) / 2;
            },
            _ => {}
        }
    }
    BoardingPass {
        row: min_row,
        col: min_col,
        id: min_row as i32 * 8 + min_col as i32 ,
    }
}

#[cfg(test)]
mod test {
    use super::{parse_line};
    use std::collections::HashSet;
    #[test]
    fn test_single() {
        parse_line("FBFBBFFRLR");
    }
    #[test]
    fn test() {
        let input = include_str!("input_day5_min.txt");
        for line in input.lines() {
            eprintln!("{:?}", parse_line(line));
        }
    }

    #[test] 
    fn part1() {
        let input = include_str!("input_day5.txt");
        let max = input.lines().map(|l|parse_line(l)).map(|b| b.id).max().unwrap_or(0);
        eprintln!("Max ID: {}", max);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day5.txt");
        let ids: HashSet<i32> = input.lines().map(|l|parse_line(l)).map(|b| b.id).collect();
        let mut potential_ids = Vec::new();
        for id in &ids {
            let n_low = id - 1;
            let n_high = id + 1;
            if !ids.contains(&n_low) || !ids.contains(&n_high) {
                potential_ids.push(id.clone());
            }
        }
        eprintln!("Potential IDs {:?}", potential_ids);
    }   
}