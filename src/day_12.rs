
fn control_ferry(input: &str, init_x: i32, init_y: i32, f_mult: i32) -> usize {
    let mut x: i32 = init_x;
    let mut y: i32 = init_y;
    let mut dir = 'E';

    input.lines().for_each(|l|{
        let (cmd, dist) = parse_command(l);
        let (new_dir, new_x, new_y) = run_command(cmd, dir, dist as i32, f_mult);
        dir = new_dir;
        x += new_x;
        y += new_y;
    });
        
    let sum = x.abs() + y.abs();
    sum as usize
}

fn parse_command(command: &str) -> (char, usize) {
    (command.chars().nth(0).unwrap_or(' '), command[1..].parse().unwrap_or(0))
}

fn run_command(cmd: char, dir: char, dist: i32, f_mult: i32) -> (char, i32, i32) {
    match cmd {
        'N' => (dir, 0, dist),
        'S' => (dir, 0, -dist),
        'E' => (dir, dist, 0),
        'W' => (dir, -dist, 0),
        'F' => run_command(dir, dir, dist*f_mult, f_mult),
        'R'|'L' => (rotate(cmd, dir, dist), 0, 0),
        _ => (dir, 0, 0),
    }
}

fn rotate(cmd: char, dir: char, angle: i32) -> char {
    let offset = angle / 90;
    let directions = ['N', 'E', 'S', 'W'];
    let (cur_index, _) = directions.iter().enumerate().find(|(_, e)| **e == dir).unwrap();
    
    let mut new_index = match cmd {
        'L' => cur_index as i32 - offset,
        'R' => cur_index as i32 + offset, 
        _ => cur_index as i32,
    };
    
    if new_index < 0 {
        new_index += directions.len() as i32;
    } else if new_index >= directions.len() as i32 {
        new_index += -(directions.len() as i32);
    }

    directions[new_index as usize]
}

#[cfg(test)]
mod tests {
    use super::control_ferry;

    #[test]
    fn part1_example() {
        let input = include_str!("example_day12.txt");
        let man_dist = control_ferry(input, 0, 0, 1);
        eprintln!("Manhaten distance of ferry: {}", man_dist);
    }

    #[test]
    fn part1() {
        let input = include_str!("input_day12.txt");
        let man_dist = control_ferry(input, 0, 0, 1);
        eprintln!("Manhaten distance of ferry: {}", man_dist);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("example_day12.txt");
        let man_dist = control_ferry(input, 10, 1, 10);
        eprintln!("Manhaten distance of ferry: {}", man_dist);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day12.txt");
        let man_dist = control_ferry(input,10, 1, 10);
        eprintln!("Manhaten distance of ferry: {}", man_dist);
    }
}