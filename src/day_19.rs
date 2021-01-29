use std::collections::HashMap;
use std::vec;

#[path =  "utility.rs"] 
mod utility;

struct Regel  
{
    rule_value : char,
    rules_collected: Vec<u32>,
}

fn process_part_1(input: &String) ->u32
{
    let mut result = 0;
    let mut rules_map = HashMap::new();
    let blank_regel = Regel{rule_value: ' ', rules_collected: Vec::new()};
    rules_map.insert(0, blank_regel);
   // println!("{}",input);
    //do some processing

    return 2;
    return result;
}

pub fn day_19()
{
    let sample_input = String::from(r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
 
ababbb
bababa
abbbab
aaabbb
aaaabbb"#);
   assert_eq!(process_part_1(&sample_input),2);
    let input = utility::get_input_from_filename(&String::from("input/day_19.txt"));
}