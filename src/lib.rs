mod day_1;

#[cfg(test)]
mod tests {
    use crate::day_1;
    use std::fs;
    #[test]
    fn day_1_test() {
        let input_string = fs::read_to_string(r"C:\Users\jjw\Documents\advent_of_code_2020\src\input_day1_part1.txt").unwrap();
        let input: Vec<i32> = input_string.lines().filter_map(|s| s.trim().parse().ok()).collect();
        if let Some(answer ) = day_1::find_answer_part1(&input) {
            eprintln!("Part1: {:?}", answer);
        }
        if let Some(answer ) = day_1::find_answer_part2(&input) {
            eprintln!("Part2: {:?}", answer);
        }
    }
}
