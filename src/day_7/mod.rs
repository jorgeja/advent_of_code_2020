use std::collections::HashMap;

type BagContents = Vec<(usize, &'static str)>;
type BagRules = HashMap::<&'static str, Option<BagContents>>;

fn parse_rules(input: &'static str) -> BagRules {
    let rules: BagRules = input.lines().filter_map(|l|{
        let tokens: Vec<&str> = l.split(" contain ").collect();                
        if tokens.len() != 2 {return None;}
        
        let name = parse_bag_name(tokens[0]);
        let mut contents = BagContents::new();

        for raw_content in tokens[1].split(", ") {                    
            let num: usize = raw_content[0..=0].parse().unwrap_or(0);
            let bag_name = parse_bag_name(raw_content[2..raw_content.len()].as_ref());
            if num > 0 {
                contents.push((num, bag_name));
            }
        }

        if contents.len() > 0 {
            return Some((name, Some(contents)));
        }
        
        Some((name, None))
    }).collect();
    rules
}

fn parse_bag_name(name: &str) -> &str {
    if let Some(index) = name.find(" bag") {
        return name[..index].as_ref();
    }
    name
}

fn find_containers_of(query: &str, rules: &BagRules) -> usize {
    let mut num_bags = 0;    
    for bag_name in rules.keys() {
        let sub_cont = find_containers_of_parent(query, *bag_name, rules);
        if sub_cont > 0 {
            num_bags += 1;
        }         
    }
    num_bags
}

fn find_containers_of_parent(query: &str, parent:&str, rules: &BagRules) -> usize {
    let mut num = 0;
    if let Some(contents) = rules.get(parent) {        
        if let Some(inner_contents) = contents {
            for (_, child) in inner_contents {
                //eprintln!("Checking against {}", child);
                if *child == query {
                    num += 1
                } else {
                    num += find_containers_of_parent(query, *child, rules)
                }
            }
        }
        
    }
    num
}

fn count_containers_of_parent(parent:&str, rules: &BagRules) -> usize {
    let mut num = 0;
    if let Some(contents) = rules.get(parent) {        
        if let Some(inner_contents) = contents {
            for (count, child) in inner_contents {
                let num_sub_bags = count_containers_of_parent(*child, rules);                                
                num += *count * (num_sub_bags + 1) ;                
            }
        }
        
    }
    num
}

#[cfg(test)]
mod test {
    use super::{find_containers_of, count_containers_of_parent,parse_rules};

    #[test]
    fn test_example() {
        let input = include_str!("example_day7.txt");        
        let rules = parse_rules(input);
        for rule in &rules {
            eprintln!("{:?}", rule);
        }

        let num_bags = find_containers_of("shiny gold", &rules);
        eprintln!("Found {} containers of shiny gold bag", num_bags)
    }
    #[test]
    fn part1() {
        let input = include_str!("input_day7.txt");        
        let rules = parse_rules(input);
        let num_bags = find_containers_of("shiny gold", &rules);
        eprintln!("Found {} containers of shiny gold bag", num_bags)
    }

    #[test]
    fn part2_example() {
        let input = include_str!("example_day7.txt");        
        let rules = parse_rules(input);
        let num_bags = count_containers_of_parent("shiny gold", &rules);
        eprintln!("Found {} containers of shiny gold bag", num_bags)
    }

    #[test]
    fn part2() {
        let input = include_str!("input_day7.txt");        
        let rules = parse_rules(input);
        let num_bags = count_containers_of_parent("shiny gold", &rules);
        eprintln!("Found {} containers of shiny gold bag", num_bags)
    }
}