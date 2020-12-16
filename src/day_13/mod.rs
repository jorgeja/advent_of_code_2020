
fn find_bus(timestamp: usize, buses: &[usize]) -> usize {
    let mut closest_arrival = (usize::MAX, usize::MAX);
    let mut iter = 0;
    let shortest_dur = *buses.iter().min().unwrap();
    loop {
        iter += 1;
        for bus_dur in buses.iter() {
            let bus_time = bus_dur * iter;
            let dur = (timestamp as i32 - bus_time as i32).abs() as usize;
            if bus_time > timestamp && dur <= *bus_dur {
                eprintln!("Bus: {} - Closest = {}", bus_dur, dur);
                if dur < closest_arrival.1 {
                    closest_arrival = (*bus_dur, dur)
                }
            }
            
            if *bus_dur == shortest_dur && bus_time - shortest_dur > timestamp {
                eprintln!("Done! {} > {} && {} - {} > {}", bus_time, shortest_dur, bus_time, shortest_dur, timestamp);
                eprintln!("{:?}", closest_arrival);
                return closest_arrival.0 * closest_arrival.1;
            }
        }
        
    }
}

fn find_timestamp(input: &[(usize, usize)], start: usize) -> usize {
    let start_val: usize = (start as f64 / input[0].1 as f64) as usize;
    eprintln!("Starting at: {}", start_val);
    for iter in start_val.. {
        let timestamp = input[0].1 * iter;
        let mut found = true;        
        for (offset, value) in input[1..].iter() {
            if (timestamp + *offset) % *value != 0 {
                found = false;
                break;
            }
        }

        if found {
          return timestamp;
        }
    }
    0
}


#[cfg(test)]
mod tests {

    use super::{find_bus, find_timestamp};

    #[test]
    fn part1_example() {
        let result = find_bus(939, &[7,13,59,31,19]);
        eprintln!("{}", result);
    }
    
    #[test]
    fn part1() {
        let input = include_str!("input_day13.txt");
        let buses: Vec<usize>  = input.split(',').filter_map(|s| s.parse::<usize>().ok()).collect();

        eprintln!("{:?}", buses);
        let result = find_bus(1000053, &buses);
        eprintln!("{}", result);
    }

    #[test]
    fn part2_example() {
        let input = "7,13,x,x,59,x,31,19";
        let parsed_input: Vec<(usize, usize)> = input.split(',').enumerate().filter_map(|(i,s)| if let Some(num) = s.parse::<usize>().ok() { Some((i, num))} else {None}).collect();
        eprintln!("{:?}", parsed_input);
        let timestamp = find_timestamp(&parsed_input, 1);
        eprintln!("Result: {}", timestamp);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day13.txt");
        let parsed_input: Vec<(usize, usize)> = input.split(',').enumerate().filter_map(|(i,s)| if let Some(num) = s.parse::<usize>().ok() { Some((i, num))} else {None}).collect();
        eprintln!("{:?}", parsed_input);
        let timestamp = find_timestamp(&parsed_input, 100000000000000);
        eprintln!("Result: {}", timestamp);
    }

    #[test]
    fn test_modulo() {
        let num = 7;
        let other_num = 3;
        
        for i in 1..10 {
            let timestamp = i*num;
            eprintln!("{} +1 = {} % {} = {}", timestamp, timestamp+1, other_num, (timestamp+1) % other_num);
        } 

    }
}