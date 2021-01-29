use std::collections::HashMap;

#[path =  "utility.rs"] 
mod utility;


fn process_x_part_2(arg :&String) -> Vec<String>
{
    let mut vector = Vec::new();
    vector.push(arg.clone());
    for i in 0..36
    {
        let character = arg.chars().nth(i).unwrap();

        if character == 'X'
        {
            let size = vector.len();
            for j in 0..size
            {
                
                let mut  alpha_string = vector[j].clone();
                alpha_string.insert(i,'1');
                alpha_string.remove(i+1);

                vector[j].insert(i,'0');
                vector[j].remove(i+1);
                vector.push(alpha_string);
            }
        }

    }

    return vector;
}

fn process_part_2( arg: &String) ->u64
{
    let mut bit_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut hash_map = HashMap::new();

    
    let  trim_string = arg.replace(&['[',']','='][..], " ");
    for line in trim_string.lines()
    {
        let mut iter = line.split_whitespace();
     
        let  val = iter.next().unwrap();
        if val == "mask"
        {
            bit_mask = iter.next().unwrap();
        }
        if val== "mem"
        {
            let key : u64 = match iter.next().unwrap().parse::<u64>()
            {
                Ok(key) => key,
                Err(_) => continue,
            };
            let value : u64 = match iter.next().unwrap().parse::<u64>() 
            {
                Ok(value) => value,
                Err(_) => continue,
            };

            //apply bitmask on this value
            //let mut value_string = fmt::Binary::fmt(&value,);
            let mut owo_value = format!("{:b}",key);
            while owo_value.len() <=35
            {
                owo_value.insert(0,'0');
            }
            for i in 0..36
            {
                let the_char = bit_mask.chars().nth(i).unwrap();
                if  the_char== '0'
                {
                   continue;
                }
                else if the_char == '1'
                {
                    owo_value.insert(i,'1');
                    owo_value.remove(i+1);
                }
                else if the_char == 'X'
                {
                    owo_value.insert(i,'X');
                    owo_value.remove(i+1);
                }
            }
            let  vector_strings = process_x_part_2(&owo_value);
            
            for i in 0.. vector_strings.len()
            {
                let muh_key = usize::from_str_radix(&vector_strings[i], 2);
                hash_map.insert(muh_key.unwrap(), value);
            }
        }
        
    }
    let mut result = 0;
    for val in hash_map.values()
    {
        result += val;
    }

    return result as u64;
}

fn process_part_1( arg: &String) -> u64
{
    let mut bit_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut hash_map = HashMap::new();

    
    let  trim_string = arg.replace(&['[',']','='][..], " ");
    for line in trim_string.lines()
    {
        let mut iter = line.split_whitespace();
     
        let  val = iter.next().unwrap();
        if val == "mask"
        {
            bit_mask = iter.next().unwrap();
        }
        if val== "mem"
        {
            let key : u64 = match iter.next().unwrap().parse::<u64>()
            {
                Ok(key) => key,
                Err(_) => continue,
            };
            let value : u64 = match iter.next().unwrap().parse::<u64>() 
            {
                Ok(value) => value,
                Err(_) => continue,
            };

            //apply bitmask on this value
            //let mut value_string = fmt::Binary::fmt(&value,);
            let mut owo_value = format!("{:b}",value);
            while owo_value.len() <=35
            {
                owo_value.insert(0,'0');
            }
            for i in 0..36
            {
                let the_char = bit_mask.chars().nth(i).unwrap();
                if  the_char== '0'
                {
                    owo_value.insert(i,'0');
                    owo_value.remove(i+1);
                }
                else if the_char == '1'
                {
                    owo_value.insert(i,'1');
                    owo_value.remove(i+1);
                }
                else if the_char == 'X'
                {
                    continue;
                }
            }
            hash_map.insert(key, usize::from_str_radix(&owo_value, 2).unwrap());
        }
        
    }
    let mut result = 0;
    for val in hash_map.values()
    {
        result += val;
    }

    return result as u64;
}

pub fn day_14()
{
    let sample_input = String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0");
    let sample_input2 = String::from("mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1");

    assert_eq!(process_part_1(&sample_input),165);
    assert_eq!(process_part_2(&sample_input2),208);

    let puzzle_input = utility::get_input_from_filename(&String::from("input/day_14.txt"));

    let result = process_part_1(&puzzle_input);
    let result_2 = process_part_2(&puzzle_input);
    println!("Day 14 result : {} |{}",result,result_2);
}