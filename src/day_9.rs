use Iterator;

#[path =  "utility.rs"] 
mod utility;

fn process_part_2(arg : &Vec<u64>, target_number: u64) -> u64
{
    let mut range= Vec::new();
    range.push(target_number);
    //iterate and figure out what range of numbers equate to target_number
    for i in 0..arg.len()
    {
        range.clear();
        let mut exit_condition =false;
        for j in i..arg.len()
        {

            range.push(arg[j]);
            let  sum_of_range: u64 =  range.iter().sum();
            if sum_of_range >target_number
            {
                break;
            }
            else if sum_of_range == target_number
            {
                exit_condition = true;
                break;
            }

        } 
        if exit_condition
        {
            break;
        }
    }

    range.sort_by(|a, b| b.cmp(a));
    println!("range smalles {} range lowest{}", range[0],range[range.len()-1]);
    let result = range[0] + range[range.len()-1];

    return result;
}

fn process_part_1(arg : &Vec<u64>, range : i32)-> u64
{
    let mut result = 0;
    let mut preamble = Vec::new();
    let mut range_counter =0;

    for number in arg
    {

        if range_counter < range
        {
            preamble.push(number);
            range_counter += 1;
            println!("Preamble length now is {}",preamble.len());
        }
        else
        {
            let mut sum_detected = false;
            for i in 0..preamble.len()
            {
                for j in i..preamble.len()
                {
                    if (preamble[i] +preamble[j]) == *number
                    {
                        sum_detected = true;
                        break;
                    }
                }
            }
            if sum_detected
            {
                // push out index 0 and push back current number in preamble
                preamble.remove(0);
                preamble.push(number);
                continue;
            }
            else
            {
                result = *number;
                break;
            }
        }
    }


    return result;
}
pub fn day_9()
{
    let sample_input= String::from("35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576");
    let sample_input_to_uint = utility::convert_to_uintegers(&sample_input);
    assert_eq!(process_part_1(&sample_input_to_uint,5),127);
    assert_eq!(process_part_2(&sample_input_to_uint,127),62);
    let input_string = utility::get_input(&String::from("done"));
    let input_to_uint = utility::convert_to_uintegers(&input_string);
    let  weakness  =process_part_1(&input_to_uint, 25);
    println!(" result is {}",weakness);
    println!("Part 2 result is{}",process_part_2(&input_to_uint, weakness));

    
}