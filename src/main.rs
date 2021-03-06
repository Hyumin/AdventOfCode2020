use std::io;
use std::vec;
use num;

mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
pub mod utility;
//process input, prints the results
fn day1_1procces_input( arg: &[i32])
{
    if arg.len() ==0
    {
        println!("can't procces zero length arrays");
        return;
    }
    let mut results = vec![];

    for  i in 0.. arg.len()-1
    {
        //Check starting from current index if any sum of the numbers are equal to 2020
        for j in i+1.. arg.len()
        {
            if arg[i]+arg[j] == 2020
            {
                results.push(arg[i]*arg[j]);
            }
        }
    }
    //display results
    for i in &results
    {
        println!("Result method 1 is: {}",i);
    }
}
//process input, prints the results
fn day1_2proccess_input( arg: &[i32])
{
    if arg.len() ==0
    {
        println!("can't procces zero length arrays");
        return;
    }
    let mut results = vec![0; 0];

    for  i in 0.. arg.len()-2
    {
        //Check starting from current index if any sum of the numbers are equal to 2020
        for j in i+1.. arg.len()-1
        {
            for k in j+1..arg.len()
            {
                if arg[i]+arg[j]+arg[k] == 2020
                {
                    results.push(arg[i]*arg[j]*arg[k]);
                }
            }
        }
    }
    //display results
    for i in &results
    {
        println!("Result method 2 is: {}",i);
    }
}


fn day_1()
{
  //Default input
  let integers = vec![1721,979,366,299,675,1456];
  day1_1procces_input(&integers);

  let mut input_integers = vec![];
  let  done_identifier = String::from("done");

  //Type custom input in the terminal to see how the function behaves with different input
  println!("Custom input, Please enter numbers in the terminal to create an array. When done type {}",done_identifier);
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

    //See if we can parse the text to a 32 bit integer, if so add it to our array
    let num : i32 = match text.trim().parse()
    {
        Ok(num)=>num,
        Err(_) =>  continue,
    };
    input_integers.push(num);
  }

  day1_1procces_input(&input_integers);
  day1_2proccess_input(&input_integers);
}

fn day2_process_string_2(arg: &str)->i32
{
    let mut return_value = 0;
    let mut iter = arg.split_whitespace();
    let mut word = String::new();
    let mut range_min = 0;
    let mut range_max = 0;
    let mut must_chars = String::new();

    //first is the range
    match iter.next()
    {
        Some(value) => word = value.to_string(),
        None => println!("yoink"),
    }

    let mut trimmed_str= word.replace(&['-'][..], " ");
    let mut iterathor = trimmed_str.split_whitespace();
    match iterathor.next()
    {
        Some(value) => range_min = value.parse::<i32>().unwrap(),
        None => println!("F"),
    }
  
    match iterathor.next()
    {
        Some(value) => range_max = value.parse::<i32>().unwrap(),
        None => println!("F"),
    }
    
    //Second is the required character(s)
    match iter.next()
    {
        Some(value) => word = value.to_string(),
        None => println!("yoink"),
    }
    must_chars = word.replace(&[':'][..], "");
    //Third is the password to itreate through
    match iter.next()
    {
        Some(value) => word = value.to_string(),
        None => println!("yoink"),
    }
    println!("password is {}",word);
    
    //There can only be one of the must char in the given range
    if word.chars().count()< (range_min-1) as usize
    {
        return return_value;
    }
    if (range_max-1) >word.chars().count() as i32
    {
        range_max = word.chars().count() as i32;
    }
    for c in must_chars.chars()
    {
        let mut value = 0;
        if c== word.chars().nth((range_min-1) as usize).unwrap()
        {
            value+= 1;
        }
        if c== word.chars().nth((range_max-1) as usize).unwrap()
        {
            value+= 1;
        }
        if value ==1
        {
            return_value+=1;
        }
    }

    return_value= num::clamp(return_value,0,1);
    return return_value;
}

//Expecting a line like this : 1-3 a: abcde
fn day2_process_string_1(arg: &str) -> i32
{
    let mut return_value = 0;
    let mut iter = arg.split_whitespace();
    let mut word = String::new();
    let mut range_min = 0;
    let mut range_max = 0;
    let mut must_chars = String::new();

    //first is the range
    match iter.next()
    {
        Some(value) => word = value.to_string(),
        None => println!("yoink"),
    }

    let mut trimmed_str= word.replace(&['-'][..], " ");
    let mut iterathor = trimmed_str.split_whitespace();
    match iterathor.next()
    {
        Some(value) => range_min = value.parse::<i32>().unwrap(),
        None => println!("F"),
    }
  
    match iterathor.next()
    {
        Some(value) => range_max = value.parse::<i32>().unwrap(),
        None => println!("F"),
    }
    
    //Second is the required character(s)
    match iter.next()
    {
        Some(value) => word = value.to_string(),
        None => println!("yoink"),
    }
    must_chars = word.replace(&[':'][..], "");
    //Third is the password to itreate through
    match iter.next()
    {
        Some(value) => word = value.to_string(),
        None => println!("yoink"),
    }
    println!("password is {}",word);
    for c in must_chars.chars()
    {
        let mut value = 0;
        for i in word.chars()
        {
            if i ==c
            {
                value += 1;
            }
        }
        if value >= range_min&& value <= range_max 
        {
            return_value += 1;
        }
        else
        {
            //Substract if its false
            return_value -=1;
        }
    }

    return_value= num::clamp(return_value,0,1);
    return return_value;
}
fn day2_2(arg: &String)
{

    let mut valid_passwords = 0;
    //Iterate on input
    //Expecting string format to be 1-3  a: abcde 
    //Range = 1-3 = minimum of 1 maximum of 3 of 
    //Requires letter =  a:  everything before the :
    //Password  is abcde 
    //Iterate the password make sure it contains at least 1-3 of the word
   

    let mut lines = arg.lines();
    for line in lines
    {
        valid_passwords += day2_process_string_2(&line);
    }
    println!("Valid passwords = {}",valid_passwords);
}
fn day2_1(arg: &String)
{

    let mut valid_passwords = 0;
    //Iterate on input
    //Expecting string format to be 1-3  a: abcde 
    //Range = 1-3 = minimum of 1 maximum of 3 of 
    //Requires letter =  a:  everything before the :
    //Password  is abcde 
    //Iterate the password make sure it contains at least 1-3 of the word
   

    let mut lines = arg.lines();
    for line in lines
    {
        valid_passwords += day2_process_string_1(&line);
    }
    println!("Valid passwords = {}",valid_passwords);
}

fn day_2()
{
    let  sample_input = String::from(
    "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc");
    println!("day 2 part 1");
    println!("Testing sample input Expected result is 2");
    day2_2(&sample_input);


  let mut input_strings = String::new();
  let  done_identifier = String::from("done123");

  //Type custom input in the terminal to see how the function behaves with different input
  println!("Custom input, Please enter input in the terminal to create a string to iterate through. When done type {}",done_identifier);
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
  day2_2(&input_strings);
}

fn main() 
{
   // println!("Day 1!");
    //Day1();
    
   // println!("Day 2!");
  //  Day2();

   // println!("Day 3!");
   //day_3::day_3();

    //day_4::day_4();

    //day_5::day_5();
    //day_6::day_6();
   day_7::day_7();
    //day_8::day_8();
    //day_9::day_9();
   // day_10::day_10();
   //day_11::day_11();
   day_12::day_12();
   day_13::day_13();
   day_14::day_14();
   day_15::day_15();
   day_16::day_16();
   day_17::day_17();
   day_18::day_18();
   day_19::day_19();
   day_21::day_21();
   day_22::day_22();
   day_23::day_23();
   day_24::day_24();
   day_25::day_25();
}
