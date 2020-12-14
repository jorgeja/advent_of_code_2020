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

fn find_steady_state_part2(seats: Vec<Vec<char>>) -> usize {
    let mut inner_seats = seats;
    let mut iter = 1;
    loop {
        eprintln!("Epoch: {}", iter);
        let (changed, new_seats) = apply_rules_part2(inner_seats);
        inner_seats = new_seats;
        iter += 1;
        if !changed{break}
    }

    inner_seats.iter().map(|v| v.iter().filter(|c| **c == '#').count()).sum()
}

fn apply_rules_part2(seats: Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut next_seat_map = seats.clone();
    let mut changed = false;
    for (r_i, row) in seats.iter().enumerate() {
        for (c_i, seat) in row.iter().enumerate() {            
            match (seat, check_neighbors_part2_two(r_i, c_i, &seats)) {
                ('L', 0) => {next_seat_map[r_i][c_i] = '#'; changed = true;},
                ('#', n) if n >= 5 => {next_seat_map[r_i][c_i] = 'L'; changed = true;},
                _ => {},
            }
        }
    }
    
    (changed, next_seat_map)
}

fn check_neighbors_part2(r:usize, c: usize, seats: &Vec<Vec<char>>) -> usize {
    let min_dr = -(r as i32);
    let max_dr = (seats.len() - 1 - r) as i32;
    let min_dc = -(c as i32);
    let max_dc = (seats[r].len() - 1 - c) as i32;    

    let mut dist_matrix: [[usize; 3]; 3] = [[usize::MAX, usize::MAX, usize::MAX], [usize::MAX, usize::MAX, usize::MAX] , [usize::MAX, usize::MAX, usize::MAX]];

    for dr in min_dr..=max_dr {
        for dc in min_dc..=max_dc {
            if seats[(r as i32 + dr) as usize][(c as i32 + dc) as usize] == '#' {
                //eprintln!("Found occupied at {}:{}, dist: {} {}", r as i32 + dr, c as i32 + dc, dr, dc);
                if dr.abs() == dc.abs() || (dr != 0 && dc == 0) || (dr == 0 && dc != 0) {
                    //eprintln!("Valid seat!");                    
                    let distance = (dr + dc).abs() as usize;
                    match (dr, dc) {
                        (i32::MIN..=-1, i32::MIN..=-1) => {
                            if distance < dist_matrix[0][0] {
                                dist_matrix[0][0] = distance;
                            }
                        },
                        (i32::MIN..=-1, 0) => {
                            if distance < dist_matrix[0][1] {
                                dist_matrix[0][1] = distance;
                            }
                        },
                        (i32::MIN..=-1, 1..=i32::MAX) => {
                            if distance < dist_matrix[0][2] {
                                dist_matrix[0][2] = distance;
                            }
                        },
                        (0, i32::MIN..=-1) => {
                            if distance < dist_matrix[1][0] {
                                dist_matrix[1][0] = distance;
                            }
                        },
                        (0, 1..=i32::MAX) => {
                            if distance < dist_matrix[1][2] {
                                dist_matrix[1][2] = distance;
                            }
                        },
                        (1..=i32::MAX, i32::MIN..=-1) => {
                            if distance < dist_matrix[2][0] {
                                dist_matrix[2][0] = distance;
                            }
                        },
                        (1..=i32::MAX, 0) => {
                            if distance < dist_matrix[2][1] {
                                dist_matrix[2][1] = distance;
                            }
                        },
                        (1..=i32::MAX, 1..=i32::MAX) => {
                            if distance < dist_matrix[2][2] {
                                dist_matrix[2][2] = distance;
                            }
                        },
                        _ => {},
                    }; 
                } else {
                    //eprintln!("Invalid Seat!")
                }
            }            
        }
    }    
    let mut num_occupied = 0;
    
    //eprintln!("{}:{} => {:?}", r, c, dist_matrix);

    for cols in dist_matrix.iter() {
        for v in cols.iter() {
            if *v < usize::MAX {
                num_occupied += 1;
            }
        }
    }
    //eprintln!("{}:{} N:{}",r,c,num_occupied);
    num_occupied
}

fn check_neighbors_part2_two(r:usize, c: usize, seats: &Vec<Vec<char>>) -> usize {
    let mut num_neigbors = 0;

    for dc in (0..c).rev() {
        if seats[r][dc] == 'L' {
            break;
        }
        else if seats[r][dc] == '#' {
            //eprintln!("Found occupied at {}:{}", r, dc);
            num_neigbors += 1;
            break;
        }
    }

    for dc in c+1..seats[r].len() {        
        if seats[r][dc] == 'L' {
            break;
        }
        else if seats[r][dc] == '#' {
            //eprintln!("Found occupied at {}:{}", r, dc);
            num_neigbors += 1;
            break;
        }
    }

    for dr in (0..r).rev() {
        if seats[dr][c] == 'L' {
            break;
        }
        else if seats[dr][c] == '#' {
            //eprintln!("Found occupied at {}:{}", dr, c);
            num_neigbors += 1;
            break;
        }
    }

    for dr in r+1..seats.len() {
        if seats[dr][c] == 'L' {
            break;
        }
        else if seats[dr][c] == '#' {
            //eprintln!("Found occupied at {}:{}", dr, c);
            num_neigbors += 1;
            break;
        }
    }

    for (dr, dc) in ((0..r).rev()).zip((0..c).rev()) {        
        if seats[dr][dc] == 'L' {
            break;
        }
        else if seats[dr][dc] == '#' {
            //eprintln!("Found occupied at {}:{}", dr, dc);
            num_neigbors += 1;
            break;
        }
    }

    for (dr, dc) in ((0..r).rev()).zip(c+1..seats[r].len()) {
        if seats[dr][dc] == 'L' {
            break;
        }
        else if seats[dr][dc] == '#' {
            //eprintln!("Found occupied at {}:{}", dr, dc);
            num_neigbors += 1;
            break;
        }
    }

    for (dr, dc) in (r+1..seats.len()).zip((0..c).rev()) {
        if seats[dr][dc] == 'L' {
            break;
        }
        else if seats[dr][dc] == '#' {
            //eprintln!("Found occupied at {}:{}", dr, dc);
            num_neigbors += 1;
            break;
        }
    }

    for (dr, dc) in (r+1..seats.len()).zip(c+1..seats[r].len()) {
        if seats[dr][dc] == 'L' {
            break;
        }
        else if seats[dr][dc] == '#' {
            //eprintln!("Found occupied at {}:{}", dr, dc);
            num_neigbors += 1;
            break;
        }
    }

    num_neigbors
}

#[cfg(test)]
mod tests {
    use super::{find_steady_state, find_steady_state_part2, check_neighbors_part2, check_neighbors_part2_two};

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

    #[test]
    fn test_eight_seats() {
        let input = include_str!("eight_seats.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        eprintln!("Seat: {}", &seats[4][3]);
        let neighbors = check_neighbors_part2_two(4, 3, &seats);
        eprintln!("Neighbors {}", neighbors);
    }

    #[test]
    fn test_no_seats() {
        let input = include_str!("no_seats.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        eprintln!("Seat: {}", &seats[3][3]);
        let neighbors = check_neighbors_part2_two(3, 3, &seats);
        eprintln!("Neighbors {}", neighbors);
    }

    #[test]
    fn test_one_seat() {
        let input = include_str!("one_seat.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        eprintln!("Seat: {}", &seats[1][3]);
        let neighbors = check_neighbors_part2_two(1, 3, &seats);
        eprintln!("Neighbors {}", neighbors);
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example_day11.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        let num_occupied = find_steady_state_part2(seats);
        eprintln!("Num occupied at end {}", num_occupied);
        //assert_eq!(num_occupied, 26);
    }
    #[test]
    fn part2_example2() {
        let input = include_str!("example2_day11_part2.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        let num_occupied = find_steady_state_part2(seats);
        assert_eq!(num_occupied, 0);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day11.txt");
        let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        let num_occupied = find_steady_state_part2(seats);
        eprintln!("Occupied seats {}", num_occupied);
    }
}