

#[path =  "utility.rs"] 
mod utility;


fn process_part_1(arg : &String) -> String
{
    let mut result = String::from("lmao");
    let input_clone = arg.clone();
    let mut arr = Vec::new();
    // Generate an 8 bit signed integer array from the input we're expecting a list of number ranging from 1 to 9
    for  c in input_clone.chars()
    {
        arr.push(c as i8 -'0' as i8);
    }
    
    let mut curr_num =  arr[0];
    let mut curr_index = 0;
    //100 steps incoming baby!!
    for i in 0..100
    {
        //First grab the 3 numbers to the right of curr_num
        let mut three_nums =  Vec::new();
        if curr_index +3 <arr.len()
        {
            three_nums = arr.drain(curr_index+1..curr_index+4).collect();
        }
        else if curr_index+2 < arr.len()
        {
            three_nums = arr.drain(curr_index+1..curr_index+3).collect();
            three_nums.push(arr[0]);
            arr.remove(0);
            
        }
        else if curr_index+1 < arr.len()
        {
            three_nums = arr.drain(0..2).collect();
            three_nums.insert(0, arr.pop().unwrap());
            
        }
        else
        {
            three_nums = arr.drain(0..3).collect();
        }

        let mut destination = curr_num-1;
        let mut destination_index = 1000;

        if destination == 0
        {
            destination = 9;
        }

        let mut destination_correct = false;

        let mut counter =0;
        while !destination_correct
        {
            if counter >= 10
            {
                println!("wtf are you doing son?!");
                break;
            }

            let find =three_nums.iter().find(|&&x| x == destination);
            
            if find == None
            {
                let index = arr.iter().position(|&x| x ==destination);
                if index != None
                {
                    destination_index = index.unwrap();
                    destination_correct = true;
                }
                
               counter +=1;
            }
            else
            {
                destination -=1;
                if destination == 0
                {
                    destination = 9;
                }
                
               counter +=1;
            }
        }
        
        for  j in 0..3
        {
            let  indy = destination_index+1+j;
            arr.insert(indy, three_nums[j]);
        }



        curr_index = arr.iter().position(|&x| x ==curr_num).unwrap()+1;

        if curr_index  >= arr.len()
        {
            curr_index =0;
        }

        curr_num = arr[curr_index];

    }
    //println!("arr length ={}",arr.len());


    return result;
}

pub fn day_23()
{
    let sample_input = String::from("389125467");

    process_part_1(&sample_input);

    let puzzle_input = utility::get_input_from_filename(&String::from("input/day_23.txt"));

    
    process_part_1(&puzzle_input);


}