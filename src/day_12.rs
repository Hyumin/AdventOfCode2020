
use num;

#[path =  "utility.rs"] 
mod utility;


fn get_value(arg: & str) -> i32
{

    let value = &arg[1..arg.chars().count()];
    let number : i32 = match value.trim().parse::<i32>() 
      {
          Ok(number) => number,
          Err(_) =>  return 0,
      };
    return number;
}

fn process_part_1(arg :&String) -> u32
{

    let mut direction = 0.0_f32;
    let mut x = 0;
    let mut y = 0;

    for line in arg.trim().lines()
    {
        //first character identifies where to go
        let identifier = line.trim().chars().nth(0).unwrap();

        let value = get_value(line);
        if identifier == 'N'
        {
            y+= value;
        }
        if identifier == 'F'
        {
            let x_maybeh = num::clamp(direction.cos(), -1.0, 1.0);
            let y_maybeh = num::clamp(direction.sin(), -1.0, 1.0);
            
             x += x_maybeh.round() as i32 * value;  
             y += y_maybeh.round() as i32 * value;
             
        }
        if identifier == 'L'
        {
            let float_val = value as f32;
            let value_in_radians = float_val.to_radians();

            direction += value_in_radians;
        }
        if identifier =='R'
        {
            let float_val = value as f32;
            let value_in_radians = float_val.to_radians();

            direction-= value_in_radians;
        }
        if identifier == 'S'
        {
            y -= value;
        }
        if identifier =='W'
        {
            x -= value;
        }
        if identifier == 'E'
        {
            x += value;
        }
    }
    let abs = i32::abs(x) +i32::abs(y);
    return abs as u32;
}

fn process_part_2(arg :&String) -> u32
{

    let mut direction = 0.0_f32;
    let mut x = 0;
    let mut y = 0;
    let mut wp_x = 10;
    let mut wp_y = 1;

    for line in arg.trim().lines()
    {
        //first character identifies where to go
        let identifier = line.trim().chars().nth(0).unwrap();

        let value = get_value(line);
        if identifier == 'N'
        {
            wp_y+= value;
        }
        if identifier == 'F'
        {
             x += value*wp_x;  
             y += value*wp_y;
             
        }
        if identifier == 'L'
        {
            let old_x = wp_x;
            let old_y = wp_y;
            if value == 90
            {
                wp_x = -old_y;
                wp_y = old_x;
            }
            if value == 180
            {
                wp_x = -old_x;
                wp_y = -old_y;
            }
            if value == 270
            {
                wp_y = -old_x;
                wp_x = old_y;
            }
        }
        if identifier =='R'
        {
            let old_x = wp_x;
            let old_y = wp_y;
            if value == 90
            {
                wp_y = -old_x;
                wp_x = old_y;
            }
            if value == 180
            {
                wp_x = -old_x;
                wp_y = -old_y;
            }
            if value == 270
            {
                wp_x = -old_y;
                wp_y = old_x;
            }
        }
        if identifier == 'S'
        {
            wp_y -= value;
        }
        if identifier =='W'
        {
            wp_x -= value;
        }
        if identifier == 'E'
        {
            wp_x += value;
        }
    }
    let abs = i32::abs(x) +i32::abs(y);
    return abs as u32;
}

pub fn day_12()
{
    let sample_input = String::from("F10
N3
F7
R90
F11");

    assert_eq!(process_part_1(&sample_input),25);
    assert_eq!(process_part_2(&sample_input),286);

    //let input = utility::get_input(&String::from("done"));
    let real_input = utility::get_input_from_filename(&String::from("input/day_12.txt"));

    print!("result day 12 = {}",process_part_1(&real_input));
    print!(" |  {}",process_part_2(&real_input));
    println!(" ");
}