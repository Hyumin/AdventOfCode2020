use std::io;
use std::vec;


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

fn part_2_what_the_nop(arg: &String, indices :Vec<i32> )-> String
{
    let return_value = String::from("nope");

    let line_count = arg.lines().count();
    let mut vector = vec![-1 as i32; 1];
    let mut fixed_at = -1;

    //brute force check if changing any of the jmp lines to nop puts us out of the loop
    for i in 0..indices.len()
    {
       // println!("index I = {}", indices[i] as i32);
        let mut j = 0;
        vector= vec![-1 as i32; 1];
        let mut  break_loop = false;
        while j < line_count as i32
        {
            for k in 0..vector.len()
            {
                if vector[k] == j
                {
                        //println!("oh no infintus loopus");
                        break_loop =true;
                        break;
                }
            }
            if break_loop
            {
                break;
            }
    
            vector.push(j as i32);
            let mut command_line = arg.lines().nth(j as usize).unwrap().split_whitespace();
            let mut command= " ";
            let mut special =false;
            match command_line.next()
            {
                Some("nop")=> 
                {
                    if j == indices[i]
                    {
                        command = "jmp";
                        special = true;
                    }
                    else
                    {
                    j+=1;
                    continue;
                    }
                },
                Some("acc")=> command = "acc",
                Some("jmp")=> command = "jmp",
                Some(value)=> command = value,
                None => print!("arf"), 
            }
    
            if command == "acc"
            {
                //parse next to integer
                let mut instruction =0;
                match command_line.next()
                {
                    Some(value)=> instruction = value.parse::<i32>().unwrap(),
                    None=> println!("something went wrong"),
                }

            }
            if command == "jmp"
            {
                let mut instruction =0;
                match command_line.next()
                {
                    Some(value)=> instruction = value.parse::<i32>().unwrap(),
                    None=> println!("something went wrong"),
                }
                j += instruction;
                continue;
            }
            if special 
            {
                println!("hello? ");
            }
            if j >= line_count as i32
            {
                println!("indices fixed at index {}",indices[i]);
                fixed_at = indices[i] as i32; //if we manage to finish the while loop normally,  we know setting this jmp to nop fixes the loop
            }
            j+= 1;
            if j >= line_count as i32
            {
                println!("indices fixed at index {}",indices[i]);
                fixed_at = indices[i] as i32; //if we manage to finish the while loop normally,  we know setting this jmp to nop fixes the loop
            }
        }

        
    }
    if fixed_at != -1 as i32
    {
        println!("correction found");
        let mut god  = String::new();

        for line in arg.lines()
        {
            if line == arg.lines().nth(fixed_at as usize).unwrap()
            {
                god.push_str(&arg.lines().nth(fixed_at as usize).unwrap().replace("nop","jmp"));
            }
            else
            {
                god += line;
            }
            god += "\n";
        }

       // println!("god {}", god.lines().nth(fixed_at as usize).unwrap().replace("jmp","nop"));
        println!("god {}",god);
        return god;
    }
    println!("no fix nop");
    return return_value;
}

fn part_2_what_the_jump(arg: &String, indices :Vec<i32> )-> String
{
    let return_value = String::from("nope");

    let line_count = arg.lines().count();
    let mut vector = vec![-1 as i32; 1];
    let mut fixed_at = -1;

    //brute force check if changing any of the jmp lines to nop puts us out of the loop
    for i in 0..indices.len()
    {
        //println!("index I = {}", indices[i] as i32);
        let mut j = 0;
        vector= vec![-1 as i32; 1];
        let mut  break_loop = false;
        while j < line_count as i32
        {
            for k in 0..vector.len()
            {
                if vector[k] == j
                {
                        //println!("oh no infintus loopus");
                        break_loop =true;
                        break;
                }
            }
            if break_loop
            {
                break;
            }
    
            vector.push(j as i32);
            let mut command_line = arg.lines().nth(j as usize).unwrap().split_whitespace();
            let mut command= " ";
            match command_line.next()
            {
                Some("nop")=> 
                {
                    j+=1;continue;
                },
                Some("acc")=> command = "acc",
                Some("jmp")=> command = "jmp",
                Some(value)=> command = value,
                None => print!("arf"), 
            }
    
            if command == "acc"
            {
                //parse next to integer
                let mut instruction =0;
                match command_line.next()
                {
                    Some(value)=> instruction = value.parse::<i32>().unwrap(),
                    None=> println!("something went wrong"),
                }

            }
            if command == "jmp"
            {
                if j != indices[i]
                {
                    let mut instruction =0;
                    match command_line.next()
                    {
                        Some(value)=> instruction = value.parse::<i32>().unwrap(),
                        None=> println!("something went wrong"),
                    }
                    j += instruction;
                    if j >= line_count as i32
                    {
                        println!("indices fixed at index {}",indices[i]);
                        fixed_at = indices[i] as i32; //if we manage to finish the while loop normally,  we know setting this jmp to nop fixes the loop
                    }
                continue;
                }
                else if j == indices[i]
                {
                    println!("skipperino");
                }
            }
            j+= 1;
            if j >= line_count as i32
            {
                println!("indices fixed at index {}",indices[i]);
                fixed_at = indices[i] as i32; //if we manage to finish the while loop normally,  we know setting this jmp to nop fixes the loop
            }
        }

    }
    if fixed_at != -1 as i32
    {
        println!("correction found");
        let mut god  = String::new();

        for line in arg.lines()
        {
            if line == arg.lines().nth(fixed_at as usize).unwrap()
            {
                god.push_str(&arg.lines().nth(fixed_at as usize).unwrap().replace("jmp","nop"));
            }
            else
            {
                god += line;
            }
            god += "\n";
        }

       // println!("god {}", god.lines().nth(fixed_at as usize).unwrap().replace("jmp","nop"));
        println!("god {}",god);
        return god;
    }
    println!("no fix");
    return return_value;
}

fn part_2(arg :&String) -> i64
{
    let mut result =0;

    let line_count = arg.lines().count();
    let mut loop_indices = vec![-1 as i32; 0];
    let mut nop_indices = vec![-1 as i32; 0];

    println!("line count{}", line_count);
    let mut i =0;
    while i < line_count as i32
    {
        let mut command_line = arg.lines().nth(i as usize).unwrap().split_whitespace();
        let mut command= " ";
       match command_line.next()
       {
           Some("nop")=> command = "nop",
           Some("acc")=> command = "acc",
           Some("jmp")=> command = "jmp",
           Some(value)=> command = value,
           None => print!("arf"), 
       }
       if command == "nop"
       {
            //parse next to integer
        let mut instruction =0;
        match command_line.next()
        {
            Some(value)=> instruction = value.parse::<i32>().unwrap(),
            None=> println!("something went wrong"),
        }
        //store any command equaling nop in the nop_indices
        if instruction > 0 || instruction <0
        {
            println!("a nop with a value hmm {} at line {} ",instruction,i);
            nop_indices.push(i as i32);
        }
       }
       if command == "jmp"
       {
           loop_indices.push(i as i32);
       }
       i +=1;
    }

    //Once broken, we want to figure out 
    let jump_checked_string=  part_2_what_the_jump(arg, loop_indices); 
    let nop_checked_string = part_2_what_the_nop(arg,nop_indices);
    println!("{}",jump_checked_string);
    println!("{}",nop_checked_string);
    if nop_checked_string != String::from("nope")
    {
        return  part_1(&nop_checked_string);
    }
    else if jump_checked_string != String::from("nope")
    {
        println!("nop");
        return part_1(&jump_checked_string);
    }
    else
    {
        return 0;
    }
}
fn part_1(arg :&String) -> i64
{
    let mut result =0;

    let line_count = arg.lines().count();
    let mut accumulator = 0;
    let mut vector = vec![-1 as i32; 1];

    let mut i =0;
    while i < line_count as i32
    {
        for j in 0..vector.len()
        {
            if vector[j] == i 
            {
                result = accumulator as i64;
                return result;
            }
        }

        vector.push(i as i32);
        let mut command_line = arg.lines().nth(i as usize).unwrap().split_whitespace();
        let mut command= " ";
       match command_line.next()
       {
           Some("nop")=> 
           {
               i+=1;continue;
           },
           Some("acc")=> command = "acc",
           Some("jmp")=> command = "jmp",
           Some(value)=> command = value,
           None => print!("arf"), 
       }

       if command == "acc"
       {
        //parse next to integer
        let mut instruction =0;
        match command_line.next()
        {
            Some(value)=> instruction = value.parse::<i32>().unwrap(),
            None=> println!("something went wrong"),
        }
        accumulator += instruction;
       // println!("acc += {} = {}",instruction,accumulator);
       }
       if command == "jmp"
       {
           let mut instruction =0;
           match command_line.next()
           {
               Some(value)=> instruction = value.parse::<i32>().unwrap(),
               None=> println!("something went wrong"),
           }
           i += instruction;
          // println!("jmp! {} i is now {}",instruction,i);
           continue;
       }
       i +=1;
    }
    result = accumulator as i64;

    return result;
}
    
pub fn day_8()
{
    println!("day 8!");
    let sample_input = String::from("nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6");
    let sample_nopt_input = String::from("nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    nop +2
    jmp -4
    acc +6");
    let done_iden = String::from("done");

    //assert_eq!(part_1(&sample_input),5);
    
    assert_eq!(part_2(&sample_input),8);
    assert_eq!(part_2(&sample_nopt_input),7);
  // println!("result part 1 = {}", part_1(&get_input(&done_iden)));
   println!("result part 2 = {}", part_2(&get_input(&done_iden)));
}