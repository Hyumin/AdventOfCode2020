#[path =  "utility.rs"] 
mod utility;





fn process_part_1( arg: &String)->u64
{
    let mut result = 0;

    let mut door_key = 0;
    let mut card_key =0;

    
    for line in arg.lines()
    {
        let number : u64 = match line.trim().parse::<u64>() 
        {
            Ok(number) => number,
            Err(_) => continue,
        };
        if door_key == 0
        {
            door_key = number;
        }
        else
        {
            card_key = number;
        }

    }

    let mut card_loop_size = 0;
    let mut door_loop_size =0;

    let mut counter = 0;
    
    let mut current_value = 1;
    loop
    {
        current_value *= 7;
        current_value  %= 20201227;

        if current_value == door_key
        {
            door_loop_size = counter+1;
        }
        if current_value == card_key
        {
            card_loop_size = counter+1;
        }
        if door_loop_size > 0 && card_loop_size >0
        {
            break;
        }

        counter +=1;
    }

    //process le number

    current_value = 1;
    for i in 0..card_loop_size
    {
        current_value *= door_key;
        current_value %= 20201227;
    }
    result = current_value;
    current_value = 1;
    for i in 0..door_loop_size
    {
        current_value *= card_key;
        current_value %= 20201227;
    }
    assert_eq!(result,current_value);

    return result;
}


pub fn day_25()
{
    let sample_input = String::from("17807724
    5764801");

    assert_eq!(process_part_1(&sample_input),14897079);

    let input = utility::get_input_from_filename(&String::from("input/day_25.txt"));

    let result = process_part_1(&input);
    println!("result day 25 | {}", result);
}