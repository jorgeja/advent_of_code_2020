use std::collections::HashMap;
fn play_game(input: &[usize], end: usize) -> usize {
    let mut seen_numbers = HashMap::new();    
    for (i, num) in input.iter().enumerate() {
        seen_numbers.insert(*num, i);
    }
           
    let mut next_num = 0;
    //eprintln!("Round {} : Spoken {}", 4, next_num);
    for round in input.len()+1..end {
        let last_num = next_num.clone();
        if let Some(last_pos) = seen_numbers.get(&next_num) {                                    
            next_num = round-1 - *last_pos;            
        } else {
            next_num = 0;
        } 
        seen_numbers.insert(last_num, round-1);

        //eprintln!("Round {} : Spoken {}", round+1, next_num);
    }

    next_num
}


#[cfg(test)]
mod tests {
    use super::play_game;

    #[test]
    fn part1_example() {
        let result = play_game(&[0, 3, 6], 10);
        assert_eq!(result, 0);
    }
        
    #[test]
    fn part1() {
        let result = play_game(&[18,8,0,5,4,1,20], 30000000);
        eprintln!("Result: {}", result);
    }
}