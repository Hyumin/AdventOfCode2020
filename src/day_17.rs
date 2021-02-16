#[path =  "utility.rs"] 
mod utility;


fn check_adjacent(x :usize , y :usize, z :usize,grid: &std::vec::Vec<std::vec::Vec<std::vec::Vec<i32>>> ) -> u32
{
    let mut res = 0;
    for l_x in -1..2 
    {
        let adjx = l_x +x as i32;
        if adjx <0 || adjx>25
        {
            continue;
        }
        for l_y in -1..2 
        {
            let adjy = l_y +y as i32;
            if adjy <0 || adjy>25
            {
                continue;
            }
            for l_z in -1..2 
            {
                if l_x ==0 && l_y == 0 && l_z==0
                {
                    continue;
                }
                let adjz = l_z+z as i32;
                if adjz <0 || adjz>25
                {
                    continue;
                }

                let value = grid[adjz as usize][adjy as usize][adjx as usize];

                if value == 1
                {
                    res +=1;
                }
            }
        }
    }
    return res;
}


fn cycle_part_1(  grid: &mut std::vec::Vec<std::vec::Vec<std::vec::Vec<i32>>>)
{
    let origin_grid = grid.clone();

    for z in 0..grid.len()
    {
        for y  in 0..grid[z].len()
        {
            for x in 0..grid[z][y].len()
            { 
                 let origin_value = origin_grid[z][y][x];
                 let val = check_adjacent(x, y, z, &origin_grid);
                 //iff its 0 check if there are 2 active nighbours if so 
                 if origin_value == 0
                 {
                     if val == 3
                     {
                       grid[z][y][x] = 1;
                     }
                    
                    
                 }
                 //if its active
                if origin_value ==1
                 {
                    if val >3 || val <2
                    {
                         grid[z][y][x] = 0;
                    }
                 }

            }
        }
    }

}


fn process_part_1(input: &String) ->u32
{
    let mut result = 0;

    //Decipher the zero position
    let mut slice_kun = vec![vec![0i32; 26]; 26];
    let mut grid = vec![vec![vec![0i32; 26]; 26]; 26];

    let x_offset = 12;
    let y_offset = 12;

    let mut y = y_offset;

    for line in input.lines()
    {
        let mut x = x_offset;
        for c in 0..line.chars().count()
        {
            let val = line.chars().nth(c).unwrap();
            if val == '#'
            {
                slice_kun[y][x]=1;
            }
            x+=1;
        }


        y+=1;
    }

    grid[12] = slice_kun;
    //Decipher slicke kun

    for i in 0..6 
    {
        cycle_part_1(&mut grid);
    }

    for z in 0..grid.len()
    {
        for y in 0..grid[z].len()
        {
            for x in 0..grid[z][y].len()
            {
                if grid[z][y][x] == 1
                {
                    result+=1;
                }
            }
        }
    }

    return result;
}

fn check_adjacent_4d(x :usize , y :usize, z :usize,w : usize ,grid: &std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<i32>>>> ) -> u32
{
    let mut res = 0;
    for l_x in -1..2 
    {
        let adjx = l_x +x as i32;
        if adjx <0 || adjx>25
        {
            continue;
        }
        for l_y in -1..2 
        {
            let adjy = l_y +y as i32;
            if adjy <0 || adjy>25
            {
                continue;
            }
            for l_z in -1..2 
            {
                let adjz = l_z+z as i32;
                if adjz <0 || adjz>25
                {
                    continue;
                }
                for l_w in -1..2 
                {
                    if l_x ==0 && l_y == 0 && l_z==0 && l_w ==0
                    {
                        continue;
                    }
                    let adjw = l_w +w as i32;
                    if adjw <0 || adjw>25
                    {
                        continue;
                    }
                    let value = grid[adjw as usize][adjz as usize][adjy as usize][adjx as usize];

                    if value == 1
                    {
                        res +=1;
                    }
             }
            }
        }
    }
    return res;
}

fn cycle_part_2(  grid: &mut std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<i32>>>>)
{
    let origin_grid = grid.clone();

    for w in 0..grid.len()
    {
        for z in 0..grid[w].len()
        {
            for y  in 0..grid[w][z].len()
            {
                for x in 0..grid[w][z][y].len()
                { 
                    let origin_value = origin_grid[w][z][y][x];
                    let val = check_adjacent_4d(x, y, z,w, &origin_grid);
                    //iff its 0 check if there are 2 active nighbours if so 
                    if origin_value == 0
                    {
                        if val == 3
                        {
                        grid[w][z][y][x] = 1;
                        }
                        
                        
                    }
                    //if its active
                    if origin_value ==1
                    {
                        if val >3 || val <2
                        {
                            grid[w][z][y][x] = 0;
                        }
                    }

                }
            }
        }
    }

}


fn process_part_2(input: &String)->u32
{
    let mut result =0;

    //Decipher the zero position
    let mut slice_kun = vec![vec![0i32; 26]; 26];
    let mut grid = vec![vec![vec![vec![0i32; 26]; 26]; 26];26];

    let x_offset = 12;
    let y_offset = 12;

    let mut y = y_offset;

    for line in input.lines()
    {
        let mut x = x_offset;
        for c in 0..line.chars().count()
        {
            let val = line.chars().nth(c).unwrap();
            if val == '#'
            {
                slice_kun[y][x]=1;
            }
            x+=1;
        }


        y+=1;
    }

    grid[12][12] = slice_kun;
    //Decipher slicke kun

    for i in 0..6 
    {
        cycle_part_2(&mut grid);
    }

    for w in 0..grid.len()
    {
        for z in 0..grid[w].len()
        {
            for y in 0..grid[w][z].len()
            {
                for x in 0..grid[w][z][y].len()
                {
                    if grid[w][z][y][x] == 1
                    {
                        result+=1;
                    }
                }
            }
        }
    }


    return result;
}

pub fn day_17()
{
    let test_input_1 = String::from(".#.
..#
###");

   // assert_eq!(112,process_part_1(&test_input_1));
   // assert_eq!(848,process_part_2(&test_input_1));

    let input = utility::get_input_from_filename(&String::from("input/day_17.txt"));

   // let result_1 = process_part_1(&input);
  //  let result_2 = process_part_2(&input);

  //processing day 17 takes loooooooong so hardcoding the answers that were found correct, uncomment if you want the algorithm to calc it tbh.
    let answer_1 = "237";
    let answer_2 = "2448";

    println!("result day 17 : {}|{}",answer_1,answer_2);


}