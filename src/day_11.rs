
#[path =  "utility.rs"] 
mod utility;



fn check_seat_occupied( arg : &Vec<i32>) ->u32
{
    
    let mut result =0;
    for c in arg {
        if *c == 2
        {
            result+=1;
        }
    }

    return result;
}
fn check_adjacent_for_hekje(arg: &Vec<i32>, x: i32 , y :i32, w :i32, h :i32) -> u32
{
    let mut result = 0;

    for i in -1..2
    {
       let pos_y = y + i;
        //Can we check above or below us ?
        if pos_y>=0 && pos_y  < h
        {
            for j in -1..2
            {
                let pos_x = x as i32  +j;
                if pos_x >= 0 && pos_x < w
                {
                    if i==0 &&j ==0
                    {
                        continue;
                    }
                    let char_to_check = arg[(pos_x+pos_y*w) as usize];
                    if char_to_check == 2
                    {
                        result+=1;
                    }
                }
                else
                {

                }
            }
        }
    }

    return result;

}

//This will change the input string according to da rules
fn apply_rules_pt_1 ( arg : &mut Vec<i32>, w: i32, h : i32 )
{

    let original = arg.clone();

    for y  in 0..h
    {
        for x in 0..w
        {
            //println!("wowx{} y{} char is {}" ,x,y,curr_char);
            let pos = w*y + x;
            let curr_value = arg[pos as usize];
            //floor
            if curr_value == 0
            {
                continue;
            }
             if curr_value == 2
            {
                //if 4 adjacent seats are # become L 
                if check_adjacent_for_hekje(&original, x, y,w,h)>=4
                {
                    arg[pos as usize] = 1;
                }
            }
             if curr_value == 1
            {
                //if 0 # seats adjacent becomes # 
                if check_adjacent_for_hekje(&original, x, y,w,h)==0
                {
                    arg[pos as usize] = 2;
                 //   println!("{}", arg);
                }
            }
           

        }
    }

}

fn check_direction_for_hekje(arg: &Vec<i32>, x: i32 , y :i32, w :i32, h :i32, dx:i32,dy:i32) ->bool
{

    let mut pos_x = x;
    let mut pos_y = y;

    pos_x += dx;
    pos_y += dy;
    if dx == 2
    {
        print!("oh oh dx");
    }
    if dy ==2
    {
        print!("oh oh  dy");
    }

    while pos_x>=0 && pos_y>=0 && pos_x < w && pos_y <h
    {
        let val = arg[(pos_x + pos_y *w) as usize];
        if val ==1
        {
            return false;
        }
        if val == 2
        {
            return true;
        }
       
        pos_y +=dy;
        pos_x +=dx;

    }

    return false;
}

fn check_line_for_hekje(arg: &Vec<i32>, x: i32 , y :i32, w :i32, h :i32) -> u32
{
    let mut result = 0;

    for i in -1..2
    {
       let pos_y = y + i;
        //Can we check above or below us ?
        if pos_y>=0 && pos_y  < h
        {
            for j in -1..2
            {
                let pos_x = x as i32  +j;
                if pos_x >= 0 && pos_x < w
                {
                    if i==0 &&j ==0
                    {
                        continue;
                    }
                    if check_direction_for_hekje(arg,x,y,w,h,j,i)
                    {
                        result+=1;
                    }
                }
                else
                {

                }
            }
        }
    }

    return result;

}
//This will change the input string according to da rules
fn apply_rules_pt_2 ( arg : &mut Vec<i32>, w: i32, h : i32 )
{

    let original = arg.clone();

    for y  in 0..h
    {
        for x in 0..w
        {
            //println!("wowx{} y{} char is {}" ,x,y,curr_char);
            let pos = w*y + x;
            let curr_value = arg[pos as usize];
            //floor
            if curr_value == 0
            {
                continue;
            }
             if curr_value == 2
            {
                //if 4 adjacent seats are # become L 
                if check_line_for_hekje(&original, x, y,w,h)>=5
                {
                    arg[pos as usize] = 1;
                }
            }
             if curr_value == 1
            {
                //if 0 # seats adjacent becomes # 
                if check_line_for_hekje(&original, x, y,w,h)==0
                {
                    arg[pos as usize] = 2;
                 //   println!("{}", arg);
                }
            }
           

        }
    }

}

fn process_part_2(arg :  &String) -> u32
{
    let mut result =0;
    let mut grid = arg.clone();
    grid = String::from(grid.trim());

    let w =grid.lines().nth(0).unwrap().chars().count();
    let h =grid.lines().count();
    let totalsize = w*h;
    println!("grid sizeY = {} size X{}, totalSize {}",w,h,w*h);
    let mut true_grid = vec![0; totalsize];

    for y in 0..h
    {
        let line = grid.lines().nth(y).unwrap();
        for x in 0..w
        {
            let c = line.chars().nth(x).unwrap();

            if c == '#'
            {
                true_grid[y*w+x] =2;
            }
            if c == 'L'
            {
                true_grid[y*w+x] =1;
            }
            if c == '.'
            {
                true_grid[y*w+x] =0;
            }
        }
    }

   

    let mut delta = 1;

    let mut counter =0;
    while delta != 0
    {
        println!("checked {}", counter);
        let prev = check_seat_occupied(&true_grid) as i64;
        apply_rules_pt_2(&mut true_grid,w as i32,h as i32);
        delta =  check_seat_occupied(&true_grid) as i64 - prev;
        counter +=1;
    }


    return check_seat_occupied(&true_grid);
}
fn process_part_1(arg :  &String) -> u32
{
    let mut result =0;
    let mut grid = arg.clone();
    grid = String::from(grid.trim());

    let w =grid.lines().nth(0).unwrap().chars().count();
    let h =grid.lines().count();
    let totalsize = w*h;
    println!("grid sizeY = {} size X{}, totalSize {}",w,h,w*h);
    let mut true_grid = vec![0; totalsize];

    for y in 0..h
    {
        let line = grid.lines().nth(y).unwrap();
        for x in 0..w
        {
            let c = line.chars().nth(x).unwrap();

            if c == '#'
            {
                true_grid[y*w+x] =2;
            }
            if c == 'L'
            {
                true_grid[y*w+x] =1;
            }
            if c == '.'
            {
                true_grid[y*w+x] =0;
            }
        }
    }

   

    let mut delta = 1;

    let mut counter =0;
    while delta != 0
    {
        println!("checked {}", counter);
        let prev = check_seat_occupied(&true_grid) as i64;
        apply_rules_pt_1(&mut true_grid,w as i32,h as i32);
        delta =  check_seat_occupied(&true_grid) as i64 - prev;
        counter +=1;
    }


    return check_seat_occupied(&true_grid);
}

pub fn day_11()
{
    let sample_input = String::from("
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL");

  assert_eq!(  process_part_1(&sample_input),37);
  assert_eq!(  process_part_2(&sample_input),26);

  let real_input = utility::get_input(&String::from("done"));

  print!("result is {}",process_part_1(&real_input));
  print!("result is {}",process_part_2(&real_input));

}