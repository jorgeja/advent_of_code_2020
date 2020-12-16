use std::collections::HashMap;

fn parse_input(input: &str) -> usize {
    let mut memory = HashMap::new();
    let mut mask = (0, 0);
    for line in input.lines() {
        if let Some(index) = line.find("mask = ") {
            mask = parse_mask(&line[index+7..]);
        } else if line.starts_with("mem") {
            let (address, new_value) = parse_value(line);
            //eprintln!("[{}] = {:#036b}", address, new_value);
            let mut value = new_value | mask.1;
            //eprintln!("VAL {:#036b}", new_value);
            //eprintln!("OR  {:#036b}", mask.1);
            //eprintln!("=   {:#036b}", value);
            value &= mask.0;            
            //eprintln!("AND {:#036b}", mask.0);
            //eprintln!("[{}] {:#036b} = {}", address, value, value);
            
            let _ = memory.insert(address, value);
        }
    }
    //eprintln!("{:?}", memory);
    memory.values().fold(0, |acc, v| acc + *v)
}

fn parse_value(input: &str) -> (usize, usize) {
    let s: Vec<&str> = input.split(" = ").collect();
    let address = s[0].strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse::<usize>().unwrap();
    let value = s[1].parse::<usize>().unwrap();
    (address, value)
}

fn parse_mask(input: &str) -> (usize, usize){
    
    let string_and_mask = input.chars().map(|c| {
        match c {
            'X' => '1',
            '1' => '1',
            _ => '0',
        }
    }).collect::<String>();
    let string_or_mask = input.chars().map(|c| {
        match c {
            '1' => '1',
            _ => '0',
        }
    }).collect::<String>();

    //eprintln!("raw mask len: {} => {}", input.len(), input);
    //eprintln!("and mask len: {} => {}", string_and_mask.len(), string_and_mask);
    //eprintln!("and   or len: {} => {}", string_or_mask.len(), string_or_mask);    

    let and_mask = usize::from_str_radix(string_and_mask.as_str(), 2).unwrap();
    let or_mask = usize::from_str_radix(string_or_mask.as_str(), 2).unwrap();
    
    //eprintln!("AND_MASK: {:#036b}", and_mask);
    //eprintln!("OR_MASK:  {:#036b}", or_mask);

    (and_mask, or_mask)
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    #[test]
    fn part1_example() {
        let input = include_str!("example_part1.txt");
        let result = parse_input(input);
        eprintln!("Sum: {}", result);
    }
    
    #[test]
    fn part1() {
        let input = include_str!("input_part1.txt");
        let result = parse_input(input);
        eprintln!("Sum: {}", result);
    } 
}