
#[path =  "utility.rs"] 
mod utility;


fn process_part_1(arg : &String)-> u32
{
    let mut result =0;
    let mut player_1_deck = Vec::new();
    let mut player_2_deck = Vec::new();

    let mut curr_player =0;

    //Extract player decks
    for line in arg.trim().lines()
    {
        if line.trim() == "Player 1:"
        {
            curr_player = 1;
            continue;
        }
        if line.trim() == "Player 2:"
        {
            curr_player = 2;
            continue;
        }
        let number : u64 = match line.trim().parse::<u64>() 
        {
            Ok(number) => number,
            Err(_) => continue,
        };
        if curr_player == 1
        {
            //Attempt to convert to number
            player_1_deck.push(number);
        }
        else if curr_player ==2
        {
             player_2_deck.push(number);
        }
    }

    while player_1_deck.len()>0 && player_2_deck.len() >0
    {
        
        //Player 1 wins
        if player_1_deck[0] > player_2_deck[0]
        {
            let winning_number = player_1_deck[0];
            let losing_number = player_2_deck[0];
            player_1_deck.remove(0);
            player_2_deck.remove(0);
            player_1_deck.push(winning_number);
            player_1_deck.push(losing_number);
        }//player 2 wins
        else if player_2_deck[0] > player_1_deck[0]
        {
            let winning_number = player_2_deck[0];
            let losing_number = player_1_deck[0];
            player_1_deck.remove(0);
            player_2_deck.remove(0);
            player_2_deck.push(winning_number);
            player_2_deck.push(losing_number);

        }//if its a draw
        else if player_1_deck[0] == player_2_deck[0]
        {
            println!("huh howd it turn out to be a draw? o.0");
        }

    }

    if player_2_deck.len() >0
    {
        let mut multiplier = player_2_deck.len();
        for i in 0..player_2_deck.len()
        {

            result +=  player_2_deck[i]as usize*multiplier;
            multiplier-= 1;
        }
    }
    if player_1_deck.len() >0
    {
        let mut multiplier = player_1_deck.len();
        for i in 0..player_1_deck.len()
        {
            result +=  player_1_deck[i] as usize*multiplier ;
            multiplier-= 1;
        }
    }

    return result as u32;
}

pub fn day_22()
{
    let test_input = String::from("
Player 1:
    9
    2
    6
    3
    1
    
    Player 2:
    5
    8
    4
    7
    10");
    assert_eq!(process_part_1(&test_input),306);
    
    let input = utility::get_input_from_filename(&String::from("input/day_22.txt"));

    println!("Day 22 result : {} |",process_part_1(&input));
}