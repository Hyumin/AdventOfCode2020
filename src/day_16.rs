use std::collections::HashMap;

#[path =  "utility.rs"] 
mod utility;

#[derive(Clone,Copy)]
struct Range
{
    min: u32,
    max: u32,
}

fn process_part_2(arg :&String)->u64
{
    let  trim_string = arg.replace(&[',',':','-'][..], " ");
    let mut hash_map = HashMap::new();
    //first lines untill your tickes will be the identifiers
    
    let mut my_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    let mut nearby_ticket_mode =false;
    let mut identify_ranges_mode = true;
    for line in trim_string.lines()
    {
        if line.trim() == ""
        {
            continue;
        }
        if !nearby_ticket_mode
        {
            if identify_ranges_mode
            {
                let mut iter = line.split_whitespace();
                let mut key = String::new();
                let mut vec = Vec::new();
                let  mut iter_val = iter.next();
                while iter_val != None
                {
                    //can we convert val to a number?
                    match iter_val
                    {
                        Some(value)=>
                        {
                            //Is value a number
                            match value.parse::<u32>()
                            {
                                Ok(n)=> 
                                {
                                    let mut range = Range{min:n, max: 1 as u32};
                                    let number : u32 = match iter.next().unwrap().parse::<u32>() 
                                    {
                                        Ok(number) => number,
                                        Err(_) => continue,
                                    };
                                    range.max = number;
                                    vec.push(range);
                                    iter_val = iter.next();
                                    continue;
                                },
                                Err(_) => print!(""),
                            };
                            //is it bags or bag
                            if value == "or"
                            {
                                iter_val = iter.next();
                                continue;
                            }
                            else if value == "your"
                            {
                                identify_ranges_mode = false;
                                //next line will be my ticket
                                break;
                            }
                            else
                            {
                                key += value;
                                key += " ";
                                iter_val = iter.next();
                            }
                        },
                        None => print!(""),
                    }
                }
                if vec.len()>0
                {
                  hash_map.insert(key, vec);
                }
            }
            else
            {
                let mut iter = line.split_whitespace();
                let mut val = iter.next();
                while val != None
                {
                    //can we convert val to a number?
                    match val
                    {
                        Some(value)=>
                        {
                            //Is value a number
                            match value.parse::<u32>()
                            {
                                Ok(n)=> 
                                {
                                    my_ticket.push(n);
                                    val = iter.next();
                                    continue;
                                },
                                Err(_) => print!(""),
                            };
                            //is it bags or bag
                            if value == "nearby"
                            {
                                nearby_ticket_mode = true;
                                break;
                            }
                        },
                        None => print!(""),
                    }
                }
            
            }
        }
        else if nearby_ticket_mode
        {
            let mut ticket = Vec::new();
            for word in line.split_whitespace()
            {
                //convert space seperated word to number
                let number : u32 = match word.parse::<u32>() 
                {
                    Ok(number) => number,
                    Err(_) => continue,
                };
                let mut valid_value =false;
                for value in hash_map.values()
                {
                    for range in value
                    {
                        if range.min <= number && range.max >= number
                        {
                            valid_value = true;
                            break;
                        }
                    }
                }
                if !valid_value
                {
                    ticket.clear();
                    break;
                }
                else
                {
                    ticket.push(number);
                }
            }
            if ticket.len() >0
            {
                nearby_tickets.push(ticket);
            }
        }

    }
  
    let mut copy_this_vec = Vec::new();
    for key in hash_map.keys()
    {
        copy_this_vec.push(key);
    }
    let mut map_places = HashMap::new();

    let ticketlength = nearby_tickets[0].len();
    while copy_this_vec.len()>0
    {
    //assuming that every ticket has the same amount of numbers
    for i in 0..ticketlength
    {
        //println!("index i {}",i);
        let mut v  = copy_this_vec.clone();
        for t in 0..nearby_tickets.len()
        {
            let num_to_compare = nearby_tickets[t][i];
            //print!("{},", num_to_compare);
            for key in hash_map.keys()
            {
                let value = hash_map.get(key).unwrap();
                let mut found =false;
                for range in value
                {
                    if range.min <= num_to_compare && range.max >= num_to_compare
                    {
                      found = true;
                    }
                    else
                    {
                       
                    }
                }
                if !found
                {
                    //remove this key from the vector
                    if let Some(index) = v.iter().position(|&r| r == key) {
                    // yeet
                    //println!("removing v index {} value {}",index,v[index]);
                    v.remove(index);
                    }
                    else {
                       
                    }
                }

            }
           // println!("counter {}",counter);
        }
        //The only thing remaining should be the key that represents this index
        if v.len()==1
        {
            map_places.insert(v[0], i);
            if let Some(index) = copy_this_vec.iter().position(|&r| r == v[0]) {
                // yeet
                copy_this_vec.remove(index);
                }
                else {
                   
                }
        }
    }
}
    let mut result = 1;
    for keys in map_places.keys()
    {
        let mut iter = keys.split_whitespace();
        let num = map_places.get(keys).unwrap().clone();

        if iter.next() == Some("departure")
        {
            result*= my_ticket[num] as u64;
        }
    }

    return result;
}

fn process_part_1(arg : &String)->u64
{
    let  trim_string = arg.replace(&[',',':','-'][..], " ");
    let mut hash_map = HashMap::new();
    let mut invalid_values = Vec::new();
    //first lines untill your tickes will be the identifiers
    
    let mut nearby_ticket_mode =false;
    let mut identify_ranges_mode = true;
    for line in trim_string.lines()
    {
        if line.trim() == ""
        {
            continue;
        }
        if !nearby_ticket_mode
        {
            if identify_ranges_mode
            {
                let mut iter = line.split_whitespace();
                let mut key = String::new();
                let mut vec = Vec::new();
                let  mut iter_val = iter.next();
                while iter_val != None
                {
                    //can we convert val to a number?
                    match iter_val
                    {
                        Some(value)=>
                        {
                            //Is value a number
                            match value.parse::<u32>()
                            {
                                Ok(n)=> 
                                {
                                    let mut range = Range{min:n, max: 1 as u32};
                                    let number : u32 = match iter.next().unwrap().parse::<u32>() 
                                    {
                                        Ok(number) => number,
                                        Err(_) => continue,
                                    };
                                    range.max = number;
                                    vec.push(range);
                                    iter_val = iter.next();
                                    continue;
                                },
                                Err(_) => print!(""),
                            };
                            //is it bags or bag
                            if value == "or"
                            {
                                iter_val = iter.next();
                                continue;
                            }
                            else if value == "your"
                            {
                                identify_ranges_mode = false;
                                break;
                            }
                            else
                            {
                                key += value;
                                iter_val = iter.next();
                            }
                        },
                        None => print!(""),
                    }
                }
                hash_map.insert(key, vec);
            }
            else
            {
                let mut iter = line.split_whitespace();
                let val = iter.next();
                if val != None
                {
                    if val.unwrap() == "nearby"
                    {
                        nearby_ticket_mode = true;
                        continue;
                    }
                }
            
            }
        }
        else if nearby_ticket_mode
        {
            for word in line.split_whitespace()
            {
                //convert space seperated word to number
                let number : u32 = match word.parse::<u32>() 
                {
                    Ok(number) => number,
                    Err(_) => continue,
                };
                let mut valid_value =false;
                for value in hash_map.values()
                {
                    for range in value
                    {
                        if range.min <= number && range.max >= number
                        {
                            valid_value = true;
                            break;
                        }
                    }
                    if valid_value
                    {
                        break;
                    }
                }
                if !valid_value
                {
                    invalid_values.push(number);
                }
            }
        }

    }
  

    let mut result = 0;
    for value in invalid_values
    {
        result+= value as u64;
    }

    return result;
}


pub fn day_16()
{
    let sample_input = String::from("
    class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50
    
    your ticket:
    7,1,14
    
    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12");
    
    assert_eq!(process_part_1(&sample_input),71);

    let input = utility::get_input_from_filename(&String::from("input/day_16.txt"));

    let result_1 = process_part_1(&input);
    let result_2 =  process_part_2(&input);

    println!("Day 16 result: {} |{}",result_1,result_2);

}