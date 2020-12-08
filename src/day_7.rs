use std::io;
use std::vec;
use regex::Regex;

//Reads the lines from the terminal and return after entering done identifier
fn get_input( done_iden : &String) -> String
{
    let mut input_strings = String::new();
    let  done_identifier = String::from(done_iden);
  
    //Type custom input in the terminal to see how the function behaves with different input
    println!("Custom input, Please enter input in the terminal to create a string to iterate through. When done type {}",done_iden);
    loop
    {
      let mut text = String::new();
  
      io::stdin().read_line(&mut text)
      .ok()
      .expect("Failed to read line");
  
      //Compare input text with doneidentifier, if true end loop
      if text.trim() == done_identifier
      {
          break;
      }
  
      input_strings+= text.trim();
      input_strings+= "\n";
    }

    return input_strings;
}
struct Bag
{
    name : String,
    can_contain : i32,
}


fn procces_part_1( arg: &String)-> i32
{
    let mut result = 0;
    let mut bags =vec![];
    let mut trim_string = arg.replace(&['.',','][..], " ");

    for line in trim_string.lines()
    {

        let mut iter = line.split_whitespace();

        match iter.next()
        {
            Some(value) => println!("next {}", value.to_string()),
            None => println!("yoink"),
        }
        match iter.next()
        {
            Some(value) => println!("next2 {}", value.to_string()),
            None => println!("yoink"),
        }
        let mut bro = String::new();
        if iter.next() != Some("bag")
        {
            match iter.next()
            {
                Some(value) => bro += value,
                None => println!("yoink"),
            }
        }

        let re = Regex::new(r"\w{5}").unwrap();

        println!("found match ? {}", re.is_match(line));

        //let mut bag = Bag{"bro" ,0};
        let mut stringyboi = String::new();
        //Replace , and dot with whitespaces so we can just look for specific words for it.
        for word in iter
        {
            //look for the word bag, and then store
            if word == "bags"|| word == "bag" 
            {
                //Somehow get were we are and split the prev 2 words from any word bag
               // bags.push(word);
                bags.push(stringyboi);
                stringyboi = String::new();
                println!("{}",word);
            }
            stringyboi += word;
        }
    }

    return result;
}


pub fn Day_7()
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