mod day_1;
mod day_2;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;

#[cfg(test)]
mod tests {
    use crate::day_1;
    use crate::day_2;
    use crate::day_4;
    #[test]
    fn day_1_test() {
        let input_string = include_str!("input_day1.txt");
        let input: Vec<i32> = input_string.lines().filter_map(|s| s.trim().parse().ok()).collect();
        if let Some(answer ) = day_1::find_answer_part1(&input) {
            eprintln!("Part1: {:?}", answer);
        }
        if let Some(answer ) = day_1::find_answer_part2(&input) {
            eprintln!("Part2: {:?}", answer);
        }
    }
    #[test]
    fn day_2_test() {
        let input_str = include_str!("input_day2.txt");
        day_2::solve(input_str);
        day_2::solve2(input_str);
    }
    #[test]
    fn day_3_test() {
        let input_str = include_str!("input_day3.txt");
        
        let slopes = vec![
            (1usize, 1usize),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2),
        ];

        
        let product = slopes.iter().map(|(right, down)| {
            let mut x = 0;
            input_str.lines()
                .step_by(*down)
                .filter(|l|{ 
                    let mut tree = false;                
                    if l.as_bytes()[x % l.len()] == b'#' {
                        tree = true;
                    }
                    x += right;
                    tree
                })
                .count()
            })
            .fold(1, |state, count|{
                eprintln!("{} * {} = {}", state, count, state * count);
                state * count
            });

        eprintln!("product {}", product);
    }
    #[test]
    fn day_4_test() {
        let input_str = include_str!("input_day4.txt");
        let passports = day_4::parse_passport(input_str);
        eprintln!("Part1: Num passports {}", passports.len());

        let passports = day_4::parse_and_validate(input_str);
        eprintln!("Part2: Num passport {}", passports.len());        
    }
}
