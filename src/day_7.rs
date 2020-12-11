use std::io;
use std::vec;
#[path =  "utility.rs"] 
mod utility;

struct string_int_pair
{
    string: String,
    integer : i32,
}
struct Bag
{
    name : String,
    can_contain : Vec<string_int_pair>,
}



fn procces_part_1( arg: &String)-> i32
{
    let mut result = 0;
    let  trim_string = arg.replace(&['.',','][..], " ");
    let mut bags = Vec::new();

    for line in trim_string.lines()
    {

        let mut iter = line.split_whitespace();
        let mut curr_bag = Bag{name: String::new(), can_contain: Vec::new()};
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
        
        println!("bag name is : {}",curr_bag.name);
        println!("next{}",iter.next().unwrap());
        let mut string_int_pair = string_int_pair{string: String::new(), integer : 0};
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
                            string_int_pair.integer = n;
                            strong = iter.next();
                            continue;
                        },
                        Err(_) => println!("nan"),
                    };
                    
                    //is it bags or bag
                    if value == "bags"|| value == "bag"
                    {
                         //curr_bag.can_contain.push(string_int_pair);
                        
                        string_int_pair.string =  String::new();
                    }
                    else
                    {
                        string_int_pair.string += value;
                    }
                },
                None => println!("endo"),
            }
            strong = iter.next();
        }
        bags.push(curr_bag);
    }
    for b in bags
    {
        print!("Bagname :{} can contains:",b.name);
        for sp in b.can_contain
        {
            print!("{} amount of {}",sp.integer,sp.string);
        }
        println!(" ");
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

    assert_eq!(procces_part_1(&sample_input),4);
}