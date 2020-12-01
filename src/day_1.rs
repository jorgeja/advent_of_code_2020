#[derive(Debug)]
pub struct Day1Answer {
    pub index_1: (usize, i32),
    pub index_2: (usize, i32),
    pub index_3: (usize, i32),
    pub product: i32,
}

pub fn find_answer_part1(input: &[i32]) -> Option<Day1Answer> {    
    for (index, elem) in input.iter().enumerate() {
        for (index_two, elem_two) in input[index..].iter().enumerate() {            
            if elem + elem_two == 2020 {                
                return Some(Day1Answer {
                    index_1: (index, elem.clone()),
                    index_2: (index_two, elem_two.clone()),
                    index_3: (0, 0),
                    product: elem * elem_two,
                })               
            }                                    
        }
    }
    None
}

pub fn find_answer_part2(input: &[i32]) -> Option<Day1Answer> {
    for (index, elem) in input.iter().enumerate() {
        for (index_two, elem_two) in input[index..].iter().enumerate() { 
            if elem + elem_two < 2020 {
                for (index_three, elem_three) in input.iter().enumerate() {
                    if elem + elem_two + elem_three == 2020 {                
                        return Some(Day1Answer {
                            index_1: (index, elem.clone()),
                            index_2: (index_two, elem_two.clone()),
                            index_3: (index_three, elem_three.clone()),
                            product: elem * elem_two * elem_three,
                        })               
                    }
                }
            }
        }
    }
    None
}