
use std::collections::HashSet;



fn compile_code(input: &str) -> Vec<(&str, Option<i64>)> {
    input.lines().filter_map(|l| {
        let tokens: Vec<&str> = l.split(' ').collect();
        if tokens.len() == 2 {
            let arg = match tokens[0] {
                "jmp"=> Some(tokens[1].parse().unwrap_or(0)),
                "acc"=> Some(tokens[1].parse().unwrap_or(0)),
                "nop"=> Some(tokens[1].parse().unwrap_or(0)),
                _=> None,
            };
            return Some((tokens[0], arg));
        }
        None
    }).collect()
}


fn run_code_unique(code: Vec<(&str, Option<i64>)>) -> i64 {
    let mut unique_ops = HashSet::new();
    let mut accumulator: i64 = 0;
    let mut pc: i64 = 0;
    loop {
        if pc as usize == code.len() {break;}

        let op = &code[pc as usize];
        //eprintln!("{:?}   => Acc:{}, PC:{}", op, accumulator, pc);
        
        if unique_ops.contains(&pc) {
            break;
        } else {
            unique_ops.insert(pc.clone());
        }

        match *op {
            ("nop", Some(_)) => pc += 1,
            ("jmp", Some(step)) => pc += step,
            ("acc", Some(num)) => {
                pc += 1;
                accumulator += num;
            },
            _ => {},
        };        
    }
    accumulator
}

fn run_code_replace(code: Vec<(&str, Option<i64>)>) -> i64 {
    let mut accumulator = 0;
    let mut pc: i64 = 0;

    loop {
        if pc as usize == code.len() {break;}

        let op = &code[pc as usize];
        
        match *op {
            ("nop", Some(step)) => {
                let (success, new_acc) = run_until_loop((pc + step) as usize,accumulator.clone(), &code);
                if success {
                    return new_acc;
                }
                pc += 1;       
            },
            ("jmp", Some(step)) => { 
                let (success, new_acc) = run_until_loop((pc + 1) as usize,accumulator.clone(), &code);
                if success {
                    return new_acc;
                } else {
                    pc += step;                
                }               
            },
            ("acc", Some(num)) => {
                pc += 1;
                accumulator += num;
            },
            _ => {},
        };
    }
    eprint!("\n\r");    
    accumulator
}

fn run_until_loop(start:usize, start_acc: i64, code: &Vec<(&str, Option<i64>)>) -> (bool, i64){
    let mut unique_ops = HashSet::new();
    let mut accumulator: i64 = start_acc;
    let mut pc: i64 = start as i64;
    loop {
        if pc as usize == code.len() {return (true, accumulator);}

        let op = &code[pc as usize];
        //eprintln!("{:?}   => Acc:{}, PC:{}", op, accumulator, pc);
        
        if unique_ops.contains(&pc) {
            return (false, accumulator);
        } else {
            unique_ops.insert(pc.clone());
        }

        match *op {
            ("nop", Some(_)) => pc += 1,
            ("jmp", Some(step)) => pc += step,
            ("acc", Some(num)) => {
                pc += 1;
                accumulator += num;
            },
            _ => {},
        };        
    }
}

#[cfg(test)]
mod test {
    use super::{compile_code, run_code_unique, run_code_replace};

    #[test]
    fn part1_example() {
        let input = include_str!("example_day8.txt");
        let code = compile_code(input);
        let result = run_code_unique(code);
        eprintln!("Acc = {}", result);
    }

    #[test]
    fn part1() {
        let input = include_str!("input_day8.txt");
        let code = compile_code(input);
        let result = run_code_unique(code);
        eprintln!("Acc = {}", result);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("example_day8.txt");
        let code = compile_code(input);
        let result = run_code_replace(code);
        eprintln!("Acc = {}", result);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day8.txt");
        let code = compile_code(input);
        let result = run_code_replace(code);
        eprintln!("Acc = {}", result);
    }
}