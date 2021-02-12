use num;
#[path =  "utility.rs"] 
mod utility;


// takes in an ID and a target and calculates which multiplication of ID is closest to target
fn return_closest_number_to_target( id:i32, target :i32) -> i32
{
    let val = target as f32/id as f32;
    return val.ceil()as i32*id ;
}

fn process_part_1(arg:&String) -> u32
{
    let mut result =0;
    // remove x and , and replace it whit whitespace so we can use the iterator
    let  trim_string = arg.replace(&['x',','][..], " ");

    let mut  first_time = true;
    let mut early_time= 0;
    let mut closest_time = 10000000;
    let mut current_id = -1;

    for line in trim_string.lines()
    {
        
        let mut iter =  line.split_whitespace();
        if first_time
        {
            let early_time_string = iter.next().unwrap();
            first_time = false;
            match  early_time_string.parse::<i32>()
            {
                Ok(n)=> 
                {
                    early_time = n;
                    continue;
                },
                Err(_) => println!("nan"),
            };
            break;
        }
        let mut  val = iter.next();

        while val != None
        {
            let mut number = 0;
            
            match  val.unwrap().parse::<i32>()
            {
                Ok(n)=> 
                {
                    number = n;
                    val = iter.next();
                },
                Err(_) => println!("nan"),
            };
            
             let time = return_closest_number_to_target(number, early_time);

             if closest_time- early_time >  time - early_time
             {
                 closest_time = time;
                 current_id = number;
             }


            val = iter.next();
      }


    }
    return (closest_time as u32 -early_time as u32) * current_id as u32;
}


fn procces_part_2(arg:&String) ->u64
{
    // remove x and , and replace it whit whitespace so we can use the iterator
    let  trim_string = arg.replace(&['x',','][..], " ");

    let mut  first_time = true;
    let mut early_time= 0;
    for line in trim_string.lines()
    {
        
        let mut iter =  line.split_whitespace();
        if first_time
        {
            let early_time_string = iter.next().unwrap();
            first_time = false;
            match  early_time_string.parse::<i32>()
            {
                Ok(n)=> 
                {
                    early_time = n;
                    continue;
                },
                Err(_) => println!("nan"),
            };
            break;
        }
        let mut  val = iter.next();

        while val != None
        {
            let mut number = 0;
            
            match  val.unwrap().parse::<i32>()
            {
                Ok(n)=> 
                {
                    number = n;
                    val = iter.next();
                },
                Err(_) => println!("nan"),
            };
             let time = return_closest_number_to_target(number, early_time);

            val = iter.next();
      }


    }


    return 0;

}

pub fn day_13()
{
    let sample_input = String::from("939
    7,13,x,x,59,x,31,19");
    let test_input_2 = String::from("123
    17,x,13,19");

   // assert_eq!(procces_part_2(&sample_input),1068781);
   // assert_eq!(procces_part_2(&test_input_2),3417);

    let input = utility::get_input_from_filename(&String::from("input/day_13.txt"));
/*
    let result_1 = process_part_1(&input);
    let result_2 = process_part_1(&input);
    println!("Day 13 result: {} |{}",result_1,result_2);*/

}