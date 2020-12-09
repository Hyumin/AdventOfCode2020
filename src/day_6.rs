#[path =  "utility.rs"] 
mod utility;


fn return_num_alphabets( arg: &String)->u32
{
    let mut result =0;
    let  a_to_z = "abcdefghijklmnopqrstuvwxyz";
    for c in a_to_z.chars()
    {
        if arg.find(c) != None
        {
            result+=1;
        }
    }
    return result;
}

fn procces_part_1(arg: &String) -> u32
{
    let mut result = 0;

    let mut remember_me = String::new();
    for line in arg.lines()
    {
        if line.chars().count() ==0
        {
            result += return_num_alphabets(&remember_me);
            remember_me = String::new();
            continue;
        }
        remember_me += line;
    }
    result += return_num_alphabets(&remember_me);
    return result;
}

//Expecting fromat like  so:
// abc
// bca
// dac
// 

fn check_similairities (arg:&String)->u32
{
    let mut first_line = String::from( arg.trim().lines().nth(0).unwrap());
    let mut i = 1;
    while first_line.chars().count() ==0
    {
        first_line = String::from( arg.trim().lines().nth(i).unwrap());
        
        i+=1;
        if i >= arg.lines().count()
        {
            return 0;
        }
    }
   // println!("Processing : {}",arg);
    for line in arg.trim().lines()
    {
         //break if we removed all characters from the first string
         if first_line.chars().count() ==0
         {
           // println!("our line is empty with this input \n {}" ,arg.trim());
             break;
         }
         //skip lines with nothing in it
        if line.chars().count() ==0
        {
           // println!("Skipping empty line with this input \n {}", arg);
            continue;
        }
       
        let mut  tmp_line = String::new();
        //println!("first line: {} :",first_line);
        for c in first_line.trim().chars()
        {
           // print!("{}",c);
            if line.find(c) == None
            {
               // tmp_line.push(c);
               // println!("Could not find {}",c)
            }
            else
            {
               // println!("Could find {}",c)
            }
            let mut shittyfound = false;
            for c2 in line.chars()
            {
                if c2 ==c
                {
                    shittyfound = true;
                }
            }
            if !shittyfound
            {
                tmp_line.push(c);
            }
        }
        for c in tmp_line.chars()
        {
            for i in 0..first_line.trim().chars().count()
            {
                if first_line.trim().chars().nth(i).unwrap() == c
                {
                    first_line.remove(i);
                    break;
                }
            }
        }
       // first_line = first_line.replace(&tmp_line, "");
    }
   return first_line.trim().chars().count() as u32;
}

fn procces_part_2(arg:&String) ->u32
{
    let mut result =0;

    let mut saved_string = String::new();

    for line in arg.lines()
    {
        if line.chars().count() ==0
        {
            println!("result = {}" ,result);
            let prevsult = result;
            result += check_similairities(&saved_string);
            if result-prevsult >=27
            {
                println!("something went wrong with {}",saved_string);
            }
           // println!("saved string {}" ,saved_string);
            saved_string = String::new();
            continue;
        }
        saved_string += line.trim();
        saved_string += "\n";//add a newline
    }
    if saved_string.trim().chars().count() > 0
    {
       println!("saved string {}" ,saved_string);
      result += check_similairities(&saved_string);
    }
    return result;
}

pub fn day_6()
{
    let sample_input = String::from("abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b");

    let test_input = String::from("
    d
    qbwhp
    r
    r");
    assert_eq!(check_similairities(&test_input),0);

  assert_eq!(  procces_part_1(&sample_input),11);
  assert_eq!(  procces_part_2(&sample_input),6);

  let input = utility::get_input(&String::from("12"));
  println!("result:{}", procces_part_1(&input));
  println!("result:{}", procces_part_2(&input));

}