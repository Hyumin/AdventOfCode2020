use std::io;
use std::vec;
use num;
use num::clamp;


fn process_part_2(arg: &String, x_vel: i32, y_vel :i32) -> i32
{
    let mut x = 0;
    let mut y =0;
    let mut y_inc = 0;

    let mut trees = 0;
    for line in arg.lines()
    {
        if y >0
        {
            if y_inc == y_vel
            {
                y_inc = 0;
                x +=x_vel;
                if x>= line.chars().count() as i32
                {
                    x = x-line.chars().count() as i32;
                }
                let curr_char =line.chars().nth(x as usize);

                if curr_char == Some('#')
                {
                    trees +=1;
                }
          }
        }

        y += 1;
        y_inc += 1;
    }
    println!("Num trees!{}",trees);
    return trees;
}

fn process_part_1( arg : &String)
{
    let mut x = 0;
    let mut y =0;

    let mut trees = 0;
    for line in arg.lines()
    {
        //Skip first check
        if(y >0)
        {
            x +=3;
            println!("le count {}", line.chars().count());
            if x>= line.chars().count()
            {
                x = x-line.chars().count();
            }
            let curr_char =line.chars().nth(x as usize);

            if curr_char == Some('#')
            {
                trees +=1;
            }
        }

        y += 1;
    }
    println!("Num trees!{}",trees);
}

pub fn day_3()
{
    let input = String::from("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#");
    process_part_2(&input,1,1);
    process_part_2(&input,3,1);
    process_part_2(&input,5,1);
    process_part_2(&input,7,1);
    process_part_2(&input,1,2);
        
    let mut input_string = String::new();
    let  done_identifier = String::from("done");

    //Type custom input in the terminal to see how the function behaves with different input
    println!("Custom input, Please enter input in the terminal to create a string to iterate through. When done type {}",done_identifier);
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

        input_string+= text.trim();
        input_string+= "\n";
    }
    let mut num :i64 = 1;
    num *= process_part_2(&input_string,1,1)as i64;
    num *= process_part_2(&input_string,3,1)as i64;
    num *= process_part_2(&input_string,5,1)as i64;
    num *= process_part_2(&input_string,7,1)as i64;
    num *= process_part_2(&input_string,1,2)as i64;

   println!("Num = {}",num);
}