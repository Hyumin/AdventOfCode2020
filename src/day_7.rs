use std::collections::HashMap;
#[path =  "utility.rs"] 
mod utility;

#[derive(Clone)]
struct StringIntPair
{
    string: String,
    integer : i32,
}
struct Bag
{
    name : String,
}


fn part_2_stuff ( key: &String,map: &HashMap<String,Vec<StringIntPair>> ) -> u32
{
    let vec = match map.get(key)
    {
        Some(vec) => vec,
        None => return 1,
    };
    if vec.len() == 0
    {
        return 1;
    }

    let mut result = 1;
    for pair in vec
    {
        result+= part_2_stuff(&pair.string, &map) * pair.integer as u32;
    }

    return result;
}


fn procces_part_2( arg: &String)-> u32
{
    let  trim_string = arg.replace(&['.',','][..], " ");
    let mut map = HashMap::new();

    for line in trim_string.lines()
    {

        let mut iter = line.split_whitespace();
        let mut curr_bag = Bag{name: String::new()};
        let mut strong = iter.next();
        while strong != Some("bags")
        {
            match strong
            {
                Some(value) =>
                { 
                    curr_bag.name += value;
                },
                None => println!("yoink"),
            }
            strong = iter.next();
        }
        
        iter.next();
        let mut vecthor = Vec::new();
        let mut pair = StringIntPair{string: String::new(), integer : 0};
        while strong != None
        {
            match strong
            {
                Some(value)=>
                {
                    //Is value a
                     match  value.parse::<i32>()
                    {
                        Ok(n)=> 
                        {
                            pair.integer = n;
                            strong = iter.next();
                            continue;
                        },
                        Err(_) => print!(""),
                    };
                    
                    //is it bags or bag
                    if value == "bags"|| value == "bag"
                    {
                         //curr_bag.can_contain.push(string_int_pair);
                        vecthor.push(pair.clone());
                        pair.string =  String::new();
                    }
                    else
                    {
                        pair.string += value;
                    }
                },
                None => print!(""),
            }
            strong = iter.next();
        }
        map.insert(curr_bag.name.clone(), vecthor);
    }

    return part_2_stuff(&String::from("shinygold"), &map)-1;
}


fn part_1_stuff(key: &String, map: &HashMap<String,Vec<StringIntPair>>) ->bool
{
    let vec = match map.get(key)
    {
        Some(vec) => vec,
        None => return false,
    };

    if vec.len()== 0
    {
        return false;
    }

    for pair in vec
    {
        if pair.string == String::from("shinygold")
        {
            return true;
        }
        else if part_1_stuff(&pair.string, &map)
        {
            return true;
        }
    }

    return false;
}

fn procces_part_1( arg: &String)-> i32
{
    let mut result = 0;
    let  trim_string = arg.replace(&['.',','][..], " ");
    let mut map = HashMap::new();

    for line in trim_string.lines()
    {

        let mut iter = line.split_whitespace();
        let mut curr_bag = Bag{name: String::new()};
        let mut strong = iter.next();
        while strong != Some("bags")
        {
            match strong
            {
                Some(value) =>
                { 
                    curr_bag.name += value;
                },
                None => println!("yoink"),
            }
            strong = iter.next();
        }
        iter.next();
        let mut vecthor = Vec::new();
        let mut pair = StringIntPair{string: String::new(), integer : 0};
        while strong != None
        {
            match strong
            {
                Some(value)=>
                {
                    //Is value a
                     match  value.parse::<i32>()
                    {
                        Ok(n)=> 
                        {
                            pair.integer = n;
                            strong = iter.next();
                            continue;
                        },
                        Err(_) => print!(""),
                    };
                    
                    //is it bags or bag
                    if value == "bags"|| value == "bag"
                    {
                         //curr_bag.can_contain.push(string_int_pair);
                        vecthor.push(pair.clone());
                        pair.string =  String::new();
                    }
                    else
                    {
                        pair.string += value;
                    }
                },
                None => print!(""),
            }
            strong = iter.next();
        }
        map.insert(curr_bag.name.clone(), vecthor);
    }
    for key in map.keys()
    {
       if part_1_stuff(key, &map)
       {
           result +=1;
       }
    }

    return result;
}


pub fn day_7()
{
    println!("Day 7");
    let sample_input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.");
    let sample_input2 = String::from("shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags.");

    assert_eq!(procces_part_1(&sample_input),4);
    assert_eq!(procces_part_2(&sample_input2),126);

    let input  = utility::get_input_from_filename(&String::from("input/day_7.txt"));

    println!("Result day 7 = {} | {}",procces_part_1(&input),procces_part_2(&input));

}