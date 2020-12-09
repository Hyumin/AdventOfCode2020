use std::io;
use std::vec;

//Reads the lines from the terminal and return after entering done identifier
fn get_input( done_iden : &String) -> String
{
    let mut inputStrings = String::new();
    let  doneIdentifier = String::from(done_iden);
  
    //Type custom input in the terminal to see how the function behaves with different input
    println!("Custom input, Please enter input in the terminal to create a string to iterate through. When done type {}",doneIdentifier);
    loop
    {
      let mut text = String::new();
  
      io::stdin().read_line(&mut text)
      .ok()
      .expect("Failed to read line");
  
      //Compare input text with doneidentifier, if true end loop
      if text.trim() == doneIdentifier
      {
          break;
      }
  
      inputStrings+= text.trim();
      inputStrings+= "\n";
    }

    return inputStrings;
}
fn process_prt_2(arg:&String)
{
    let mut ids = vec![0; 0];
    for line in arg.lines()
    {
        let mut row_min = 0;
        let mut row_max =127;
        let mut row = 0;

        let mut col = 0;
        let mut col_min= 0;
        let mut col_max = 7;
        
        for c in line.chars()
        {
           // println!("Row min {} ,Row max {} ",row_min,row_max);
            //means to take the upper half
            if c == 'B'
            {
                let diff = row_max-row_min+1;
                if diff ==2
                {
                    row = row_max;
                }
                row_min +=diff/2;
            }
            // means to take the lower half
            if c == 'F'
            {
                let diff = row_max-row_min+1;
                if diff ==2
                {
                    row = row_min;
                }
                
                row_max -= diff/2;
            }
            // upper half
            if c== 'R'
            {
                let diff = col_max-col_min+1;
                println!("col_min {} col_max {}",col_min,col_max);
                if diff ==2
                {
                    col = col_max;
                }
                col_min += diff/2;
                
            }
            //lower half 
            if c == 'L'
            {
                let diff = col_max-col_min+1;

                println!("col_min {} col_max {}",col_min,col_max);
                if diff ==2
                {
                    col = col_min;
                }
                col_max -=  diff/2;
            }


        }
        let result = (row*8) + col;

        ids.push(result);
    }
    
    for x in 0..127
    {
        for y in 0..7
        {
        let  i = x*8 + y;
        let mut found = false;
        for j in 0..ids.len()
        {
          if i == ids[j]
          {
             found = true;
          }   
        }
        if !found
        {
            let mut hmm = 0;
            if x >0 && x < 127
            {
                if y>0 && y<7
                {
                    for j in 0..ids.len()
                    {
                        //check if next index exists
                    if i+1 == ids[j]
                    {
                        hmm += 1;
                    }
                    //check if preivous index exists
                    if i-1 == ids[j]
                    {
                        hmm+=1;
                    }   
                }
            }
            }
            if hmm ==2
            {
                println!("my seat at : {}",i);
            }

        }
        }
    }
    
}

fn process_prt_1(arg:&String)-> i32
{
    let mut result = 0;

    for line in arg.lines()
    {
        let mut row_min = 0;
        let mut row_max =127;
        let mut row = 0;

        let mut col = 0;
        let mut col_min= 0;
        let mut col_max = 7;
        
        for c in line.chars()
        {
           // println!("Row min {} ,Row max {} ",row_min,row_max);
            //means to take the upper half
            if c == 'B'
            {
                let diff = row_max-row_min+1;
                if diff ==2
                {
                    row = row_max;
                }
                row_min +=diff/2;
            }
            // means to take the lower half
            if c == 'F'
            {
                let diff = row_max-row_min+1;
                if diff ==2
                {
                    row = row_min;
                }
                
                row_max -= diff/2;
            }
            // upper half
            if c== 'R'
            {
                let diff = col_max-col_min+1;
                println!("col_min {} col_max {}",col_min,col_max);
                if diff ==2
                {
                    col = col_max;
                }
                col_min += diff/2;
                
            }
            //lower half 
            if c == 'L'
            {
                let diff = col_max-col_min+1;

                println!("col_min {} col_max {}",col_min,col_max);
                if diff ==2
                {
                    col = col_min;
                }
                col_max -=  diff/2;
            }
        }
        println!("Row:{}",row);
        println!("Col:{}",col);
        let  temp_result = (row *8) + col;
        println!("result = {}",temp_result);

        if temp_result>result
        {
            result= temp_result;
        }
        
    }

    return result;
}


pub fn day_5()
{
    println!("day 5");
    let defautltInput = String::from("FBFBBFFRLR
    BFFFBBFRRR
    FFFBBBFRRR
    BBFFBBFRLL");

    println!("Expecting 820: answer is : {} ",process_prt_1(&defautltInput));
    process_prt_2(&get_input(&String::from("done")));
    //println!("output from input expecting 965 {}",process_prt_1(&get_input(&String::from("done"))));

}