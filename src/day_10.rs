#[path =  "utility.rs"] 
mod utility;


fn process_part_1(arg :&String)-> u64
{
    let mut vector = utility::convert_to_uintegers(arg);
    vector.sort_by(|a, b| a.cmp(b));
    
    let mut diff_3 = 1;
    let mut diff_2 =0;
    let mut diff_1 =0;
    let mut cur_num = 0;

    for i in 0..vector.len()
    {
        let number = vector[i];
        let diff =number-cur_num;
        if diff <= 3
        {
            match diff
            {
                0 => (),
                1=> diff_1 +=1,
                2=> diff_2 +=1,
                3=>diff_3 +=1,
                _ => (),
            }
            cur_num = number;
        }
        else
        {
            println!("invalid");
        }
    }


    return diff_1*diff_3;
}


fn path_finding (arg:&Vec<u32>,index : u32)-> u64
{
    let mut result =0;
    let mut curr_num =arg[index as usize];
    //println!("pathfinder index{}",index);
   /* if (index+1) < arg.len() as u64
    {
        for i in (index+1) as usize..arg.len()
        {
            let diff = arg[i] - curr_num;
            if diff <= 3
            { 
                //Check 1 numbers ahead
                if  i < arg.len()-1
                {
                    if arg[i+1]-curr_num <=3
                    {
                        result+= path_finding(arg, (i+1) as u64);
                    }
                }
                //Check 2 numbers ahead
                if  i < arg.len()-2
                {
                    if arg[i+2]-curr_num <=3
                    {
                        result+= path_finding(arg, (i+2) as u64);
                    }
                }

                curr_num = arg[i];
            }
            else
            {
                //if our next is not valid return 0
                println!("no path here sir");
                return 0;
            }
        }
    }*/
    
   
        let i = index as usize;
        if i == arg.len()-1
        {
            result +=1;
            return result;
        }
        //Check 1 numbers ahead
        if  i < arg.len()-1
        {
            if arg[i+1]-curr_num <=3
            {
                result+= path_finding(arg, (i+1) as u32);
            }
        }
        //Check 2 numbers ahead
        if  i < arg.len()-2
        {
            if arg[i+2]-curr_num <=3
            {
                result+= path_finding(arg, (i+2) as u32);
            }
        }
        //Check 3 numbers ahead
        if  i < arg.len()-3
        {
            if arg[i+3]-curr_num <=3
            {
                result+= path_finding(arg, (i+3) as u32);
            }
        }
    //if we reach the end of the loop succesfully we should get here and increment
    //result by 1 
    //println!("max has been hit adding one!");

    return result;

}

fn process_part_2(arg :&String) -> u64
{
    let mut vector = utility::convert_to_uintegers_32(arg);
    vector.push(0);
    vector.sort_by(|a, b| a.cmp(b));
    
    println!("length is {}",vector.len());
   
    //let num_changes = path_finding(&vector, 0);

    let mut memo = vec![0 as u64;vector.len()];
    memo[0] =1;
    for i in 1..vector.len()
    {
        let number = vector[i];
        let mut  diff =number - vector[i-1];
        if diff<= 3
        {
            memo[i]+= memo[i-1];
        }
        //Check 1 numbers before
        if  i >1 
        {
            diff =number-vector[i-2] ;
            if diff <=3
            {
                memo[i]+= memo[i-2];
            }
        }
        //Check 2 numbers before
        if  i >2
        { 
            diff =number-vector[i-3];
            if diff<=3
            {
                memo[i]+= memo[i-3];
            }
        }
    }
  
    let num_changes = memo.last().unwrap();
    return *num_changes;
}

pub fn day_10()
{
    print!("day 10!");
    let small_example = String::from("16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4");
    let large_example = String::from("28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3");

    assert_eq!(process_part_1(&small_example),7*5);
    assert_eq!(process_part_1(&large_example),22*10);
    assert_eq!(process_part_2(&small_example),8);
    assert_eq!(process_part_2(&large_example),19208);

    let input = utility::get_input(&String::from("done"));

    println!("result {}", process_part_1(&input));

    println!("result2 {}", process_part_2(&input));
}