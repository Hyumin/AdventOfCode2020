use std::io;
use std::vec;

//process input, prints the results
fn Day1_1ProccessInput( _arg: &[i32])
{
    if _arg.len() ==0
    {
        println!("can't procces zero length arrays");
        return;
    }
    let mut results = vec![0; 0];

    for  i in 0.. _arg.len()-1
    {
        //Check starting from current index if any sum of the numbers are equal to 2020
        for j in i+1.. _arg.len()
        {
            if _arg[i]+_arg[j] == 2020
            {
                results.push(_arg[i]*_arg[j]);
            }
        }
    }
    //display results
    for i in &results
    {
        println!("Result is: {}",i);
    }
}


fn Day1_1()
{
  //Default input
  let integers = vec![1721,979,366,299,675,1456];
  Day1_1ProccessInput(&integers);

  let mut inputIntegers = vec![0; 0];
  let  doneIdentifier = String::from("done");

  //Type custom input in the terminal to see how the function behaves with different input
  println!("Custom input, Please enter numbers in the terminal to create an array. When done type {}",doneIdentifier);
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

    //See if we can parse the text to a 32 bit integer, if so add it to our array
    let num : i32 = match text.trim().parse()
    {
        Ok(num)=>num,
        Err(_) =>  continue,
    };
    inputIntegers.push(num);
  }

  Day1_1ProccessInput(&inputIntegers);

}

fn main() 
{
    println!("Day 1 puzzle 1!");
    Day1_1();
}
