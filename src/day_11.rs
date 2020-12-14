fn find_steady_state(seats: Vec<Vec<char>>) -> usize {
    let mut inner_seats = seats;
    loop {
        let (changed, new_seats) = apply_rules(inner_seats);
        inner_seats = new_seats;
        if !changed{break}
    }

    inner_seats.iter().map(|v| v.iter().filter(|c| **c == '#').count()).sum()
}

fn apply_rules(seats: Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut next_seat_map = seats.clone();
    let mut changed = false;
    for (r_i, row) in seats.iter().enumerate() {
        for (c_i, seat) in row.iter().enumerate() {            
            match (seat, check_neighbors(r_i, c_i, &seats)) {
                ('L', 0) => {next_seat_map[r_i][c_i] = '#'; changed = true},
                ('#', n) if n >= 4 => {next_seat_map[r_i][c_i] = 'L'; changed = true},
                _ => {},
            }
        }
    }
    
    (changed, next_seat_map)
}

fn check_neighbors(r:usize, c: usize, seats: &Vec<Vec<char>>) -> usize {
    let min_r = if r == 0 { 0 } else { r - 1};
    let max_r = if r == seats.len() - 1 { r } else {r + 1};
    let min_c = if c == 0 { 0 } else { c - 1};
    let max_c = if c == seats[r].len() - 1 { c } else { c + 1 };
    let mut num_occupied = 0;
    for cr in min_r..=max_r {
        for cc in min_c..=max_c {
            if cr == r && cc == c { continue };
            match seats[cr][cc] {
                '#' => num_occupied += 1,
                _ => {}
            }
        }
    }
    num_occupied
}


#[cfg(test)]
mod tests {
    use super::find_steady_state;

    #[test]
    fn part1_example() {
        let input = include_str!("example_day11.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        let num_occupied = find_steady_state(seats);
        eprintln!("Occupied seats {}", num_occupied);
    }

    #[test]
    fn part1() {
        let input = include_str!("input_day11.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        let num_occupied = find_steady_state(seats);
        eprintln!("Occupied seats {}", num_occupied);
    }
}