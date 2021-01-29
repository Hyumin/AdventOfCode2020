
#[path =  "utility.rs"] 
mod utility;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector2
{
    x : i32,
    y : i32,
}




fn process_part_1(arg : &String) ->u32
{
    let mut gridje= vec![vec![1i32;100];100];

    //start at the half of the array so we can just apply a minust operator with mostly no wrory lmao
    let offsetpoint = 50;

    for line in arg.lines()
    {
        let mut vector_two = Vector2{ x: offsetpoint , y : offsetpoint};
        let mut prev_char = 'l';
        for c in line.chars()
        {
            let type_acces = vector_two.y %2;
            if c == 'e'
            {
                if prev_char == 'n'
                {
                    if type_acces == 0
                    {
                        vector_two.y -=1;
                    }
                    else if type_acces ==1
                    {
                        vector_two.x +=1;
                        vector_two.y -=1;
                    }
                }
                else if prev_char == 's'
                {
                    if type_acces ==0
                    {
                        vector_two.y +=1;
                    }
                    else if type_acces ==1
                    {
                        vector_two.x +=1;
                        vector_two.y +=1;
                    }
                }
                else
                {
                    vector_two.x += 1;
                }
            }
            else if c == 'w'
            {
                if prev_char == 'n'
                {
                    if type_acces == 0
                    {
                        vector_two.x -=1;
                        vector_two.y -=1;
                    }
                    else if type_acces ==1
                    {
                        vector_two.y -=1;
                    }
                }
                else if prev_char == 's'
                {
                    if type_acces ==0
                    {
                        vector_two.x -=1;
                        vector_two.y +=1;
                    }
                    else if type_acces ==1
                    {
                        vector_two.y +=1;
                    }
                }
                else
                {
                    vector_two.x -= 1;
                }
            }
            prev_char = c;
        }

        gridje[vector_two.x as usize][vector_two.y as usize] *=-1;

        //println!("vector2({},{})",vector_two.x,vector_two.y);


    }

    let mut result =0;
    //count minus ones
    for y in 0..100
    {
        for x in 0..100
        {
            if gridje[x][y] == -1
            {
                result += 1;
            }
            //print!("{}",gridje[x][y]);
        }
        
        //println!();
    }

    return result;
}


pub fn day_24()
{
    let sample_input = String::from(
    "sesenwnenenewseeswwswswwnenewsewsw
    neeenesenwnwwswnenewnwwsewnenwseswesw
    seswneswswsenwwnwse
    nwnwneseeswswnenewneswwnewseswneseene
    swweswneswnenwsewnwneneseenw
    eesenwseswswnenwswnwnwsewwnwsene
    sewnenenenesenwsewnenwwwse
    wenwwweseeeweswwwnwwe
    wsweesenenewnwwnwsenewsenwwsesesenwne
    neeswseenwwswnwswswnw
    nenwswwsewswnenenewsenwsenwnesesenew
    enewnwewneswsewnwswenweswnenwsenwsw
    sweneswneswneneenwnewenewwneswswnese
    swwesenesewenwneswnwwneseswwne
    enesenwswwswneneswsenwnewswseenwsese
    wnwnesenesenenwwnenwsewesewsesesew
    nenewswnwewswnenesenwnesewesw
    eneswnwswnwsenenwnwnwwseeswneewsenese
    neswnwewnwnwseenwseesewsenwsweewe
    wseweeenwnesenwwwswnew");

   assert_eq!( process_part_1(&sample_input),10);

    let puzzle_input = utility::get_input_from_filename(&String::from("input/day_24.txt"));

    
   let result = process_part_1(&puzzle_input);

    println!("Day 24 result : {}|",result);
}