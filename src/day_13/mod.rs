
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


#[cfg(test)]
mod tests {

    use super::find_bus;

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
}