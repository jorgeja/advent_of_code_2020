
fn control_ferry(input: &str) -> usize {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = 'E';

    input.lines().for_each(|l|{
        let (cmd, dist) = parse_command(l);
        let (new_dir, new_x, new_y) = run_command(cmd, dir, dist);
        dir = new_dir;
        x += new_x;
        y += new_y;
    });
        
    let sum = x.abs() + y.abs();
    sum as usize
}

fn control_ferry_by_waypoint(input: &str, wp_init_x: i32, wp_init_y: i32) -> usize {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut wp_x: i32 = wp_init_x;
    let mut wp_y: i32 = wp_init_y;    

    input.lines().for_each(|l|{
        let (cmd, dist) = parse_command(l);
        if cmd != 'F' {
            let (new_x, new_y) = run_waypoint_command(cmd, dist, wp_x, wp_y);            
            wp_x = new_x;
            wp_y = new_y;
        } else if cmd == 'F' {
            x += wp_x * dist;
            y += wp_y * dist;
        }
        eprintln!("{} => wp {},{} : f {}, {}", l, wp_x, wp_y, x, y);        
    });
        
    let sum = x.abs() + y.abs();
    sum as usize
}

fn parse_command(command: &str) -> (char, i32) {
    (command.chars().nth(0).unwrap_or(' '), command[1..].parse().unwrap_or(0))
}

fn run_command(cmd: char, dir: char, dist: i32) -> (char, i32, i32) {
    match cmd {
        'N' => (dir, 0, dist),
        'S' => (dir, 0, -dist),
        'E' => (dir, dist, 0),
        'W' => (dir, -dist, 0),
        'F' => run_command(dir, dir, dist),
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

fn run_waypoint_command(cmd: char, dist: i32, x: i32, y: i32) -> (i32, i32) {
    match cmd {
        'N' => (x, y+dist),
        'S' => (x, y-dist),
        'E' => (x+dist, y),
        'W' => (x-dist, y),
        'L' => rotate_coords(dist as f32, x, y),
        'R' => rotate_coords(-dist as f32, x, y),
        _ => (x, y),
    }
}

fn rotate_coords(angle: f32, x: i32, y: i32) -> (i32, i32) {
    let (s_a, c_a) = angle.to_radians().sin_cos();    
    ((x as f32 * c_a) as i32 - (y as f32 * s_a) as i32, (x as f32 * s_a) as i32 + (y as f32 * c_a) as i32)
}

#[cfg(test)]
mod tests {
    use super::{control_ferry, control_ferry_by_waypoint};

    #[test]
    fn part1_example() {
        let input = include_str!("example_day12.txt");
        let man_dist = control_ferry(input);
        eprintln!("Manhatten distance of ferry: {}", man_dist);
    }

    #[test]
    fn part1() {
        let input = include_str!("input_day12.txt");
        let man_dist = control_ferry(input);
        eprintln!("Manhatten distance of ferry: {}", man_dist);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("example_day12.txt");
        let man_dist = control_ferry_by_waypoint(input, 10, 1);
        eprintln!("Manhatten distance of ferry: {}", man_dist);
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day12.txt");
        let man_dist = control_ferry_by_waypoint(input,10, 1);
        eprintln!("Manhatten distance of ferry: {}", man_dist);
    }
}