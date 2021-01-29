use std::vec;
#[path =  "utility.rs"] 
mod utility;

fn process_part_2( arg: &String) -> u32
{
    let mut arr = vec![0; 30000000];
    //Decipher the first X amount of numbers in the array
    let  trim_string = arg.replace(&[','][..], " ");

    let mut curr_index = 0;
    for word in trim_string.split_whitespace()
    {
        let number : u32 = match word.trim().parse::<u32>() 
      {
          Ok(number) =>number,
          Err(_) => continue,
      };
      arr[curr_index] =number;
      curr_index+=1;
    }
    let mut prev = 0;
    let mut found = false;
    for i in curr_index..30000000
    {
        //println!("{}",i);
        let index = i-1;
        let current_num = arr[index];
        found = false;
        for j in (0..index).rev()
        {
            if arr[j] == current_num
            {
                prev = j as u32;
                found = true;
                break;
            } 
        }
        if found
        {
            arr[i] = index as u32 - prev;
        }
        else
        {
            arr[i] = 0;
        }

    }
    return arr[29999999];
}
fn process_part_1( arg: &String) -> i32
{
    let mut arr : [i32; 2020] = [0; 2020];
    //Decipher the first X amount of numbers in the array
    let  trim_string = arg.replace(&[','][..], " ");

    let mut curr_index = 0;
    for word in trim_string.split_whitespace()
    {
        let number : i32 = match word.trim().parse::<i32>() 
      {
          Ok(number) =>number,
          Err(_) => continue,
      };
      arr[curr_index] =number;
      curr_index+=1;
    }

    for i in curr_index..2020
    {
        if arr[i] >0
        {
          //println!("{}",arr[i]);
        }

        if i ==0
        {
            continue;
        }
        let current_num = arr[i-1];
        let mut prev = -1;
        let mut found = false;
        for j in (0..(i-1)).rev()
        {
            if arr[j] == current_num
            {
                prev = j as i32;
                found = true;
                break;
            } 
        }
        if found
        {
            arr[i] = (i-1) as i32 - prev;
        }
        else
        {
            arr[i] = 0;
        }

    }

    return arr[2019];
}

pub fn day_15()
{
    let sample_input = String::from("0,3,6");
    let sample_input2 = String::from("1,3,2");
    let sample_input3 = String::from("3,1,2");

    assert_eq!(process_part_1(&sample_input),436);//change to 165 later
    assert_eq!(process_part_1(&sample_input2),1);//change to 165 later
    assert_eq!(process_part_1(&sample_input3),1836);//change to 165 later

    let input = String::from("12,1,16,3,11,0");
   // let result_pt_2 = process_part_2(&input);
    println!("Day 15 result = {} |{}",process_part_1(&input),"OwO");

}