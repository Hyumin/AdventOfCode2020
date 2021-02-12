#[path =  "utility.rs"] 
mod utility;

//This will expect a string that looks like this 10+5) or 10+10*(5+10))
//


pub fn next_operation(arg: &String, pos: usize)->usize
{
    for i in pos..arg.chars().count()
    {
        let c = arg.chars().nth(i).unwrap();
        if c == '+' || c == '*' || c == ')' 
        {
            return i;
        }
    }
    return 0;
}

pub fn execute_opeartion( op_c: char, l: u128, r:u128)->u128
{
    if op_c == '+'
    {
        return l+r;
    }
    else if op_c == '*'
    {
        return l*r;
    }
    else
    {

    }
    return 0;
}
pub fn execute_opeartion_1( op_c: char, l: &String, r:&String)->u128
{
    let n_l : u128 = match l.parse::<u128>() 
    {
        Ok(n_l) => n_l,
        Err(_) => return 0,
    };
    let n_r : u128 = match r.parse::<u128>() 
    {
        Ok(n_r) => n_r,
        Err(_) => return 0,
    };

    if op_c == '+'
    {
        return n_l+n_r;
    }
    else if op_c == '*'
    {
        return n_l*n_r;
    }
    else
    {

    }
    return 0;
}

//return position of the next ) return 0 if nothing found
pub fn detect_close_parenthesis(arg: &String, pos: usize) ->usize
{
    let mut close_to_ignore = 0;
    for i in pos..arg.chars().count()
    {
        let c = arg.chars().nth(i).unwrap();

        if c == '('
        {
            close_to_ignore += 1;
        }
        if c == ')'
        {
            if close_to_ignore ==0
            {
                return i;
            }
            else
            {
                close_to_ignore -=1;
            }
        }
    }

    return 0;
}

fn calc_parenthesis(arg: &String, pos: usize) -> u128
{
    let mut result =0;
    let mut current_operation = ' ';
    let mut current_left= 0;
    let mut first_left = String::from("");
    let mut current_right = String::from("");

    let mut i =pos;
    while  i < arg.chars().count()
    {
        let c= arg.chars().nth(i).unwrap();
        if c == '+' || c == '*'
        {
            let mut op_res = 0;
          
            if current_operation!= ' '
            {
                
                if current_left!= 0
                {
                    let n_r : u128 = match current_right.parse::<u128>() 
                    {
                        Ok(n_r) => n_r,
                        Err(_) => return 0,
                    };
                    op_res = execute_opeartion(current_operation,current_left,n_r);
                    //println!("left {}  {} right{}",current_left,current_operation,current_right);
                    current_right.clear();
                }
                else
                {
                    op_res = execute_opeartion_1(current_operation,&first_left,&current_right);
                    current_right.clear();
                }
             }
             if op_res == 0
             {
                //do nothing
             }
             else
             {
                result =op_res;
                current_left = result;
             }
            //operation will be 
            current_operation = c;


        }
        else if c== '('
        {
            //figure out where the next ) is  and calculate the stuff in the parenthesis
            let pos_to_skip =detect_close_parenthesis(arg, i+1);
            let right_hand = calc_parenthesis(arg,i+1);
            //convert righthand and lefthand to proper numbers
            let mut op_res = execute_opeartion(current_operation,current_left,right_hand);
            if current_left == 0 && first_left.len()>0
            {
                op_res = execute_opeartion_1(current_operation,&first_left,&right_hand.to_string());
            }
            else if current_left ==0
            {
                // we need to make current_left the result of the righthand instead
                current_left= right_hand;
            }
            // increment i to next )
            if op_res == 0
            {
               //do nothing
            }
            else
            {
               result =op_res;
               current_left = result;
            }
           //operation will be 
           current_operation = ' ';

           i = pos_to_skip+1;
           continue;
        }
        else if c == ')'
        {
            if current_right.len()>0
            {
                if current_operation!= ' '
                {
                    //execute last operation
                    let mut op_res = execute_opeartion_1(current_operation,&current_left.to_string(),&current_right);
                    if current_left == 0
                    {
                        op_res = execute_opeartion_1(current_operation,&first_left,&current_right);
                    }
                    result = op_res;
                    current_operation = ' ';
                }
            }
            break;
        }
        else if c != ' '
        {
            if current_operation ==' '
            {
                first_left.push(c);
            }
            else
            {
                current_right.push(c);
            }
        }
        i +=1;
    }
    
    if current_operation != ' '
    {
        if current_left !=0
        {

            let n_r : u128 = match current_right.parse::<u128>() 
            {
                Ok(n_r) => n_r,
                Err(_) => return 0,
            };
            let  op_res = execute_opeartion(current_operation,current_left,n_r);
            result = op_res;
         }
         else
         {
            let op_res = execute_opeartion_1(current_operation, &first_left, &current_right);
            result = op_res;
         }
    }
    return result;
}



fn calc_parenthesis_plus(arg: &String, pos: usize) -> u128
{
    let mut result =0;
    let mut current_operation = ' ';
    let mut current_left= 0;
    let mut current_right = 0;

    let mut i =pos;
    
    while i < arg.chars().count()
    {
        let c = arg.chars().nth(i).unwrap();
        if c == ' '|| c== '+'||c=='*'||c=='('||c==')'
        {
            if c == '+'
            {   
                //if a current operation has been set execute it 
                if current_operation != ' '
                {
                    if current_operation == '*'
                    {
                        //we need to handle the plus first so lhnd + rhnd
                        let mut lhand_2 = current_right;
                        let mut rhand_2 = 0;
                        let mut j = i+1;
                        let mut c2 = arg.chars().nth(j).unwrap();
                        //use j to detect next number
                        while c2 == ' '
                        {
                            j+=1;
                            c2 = arg.chars().nth(j).unwrap();
                        }
                        //c2 should be a  number
                        let number = match c2.to_string().parse::<u128>()
                        {
                            Ok(number)=> number,
                            Err(_)=> 0,
                        };
                        if number != 0
                        {
                            rhand_2 = number;
                        }
                        if number == 0
                        {
                            if c2 == '('
                            {
                                //do the parenthesis electric boogaloo
                                //basically change j to the parenthesis
                                let pos_to_skip =detect_close_parenthesis(arg, j+1);
                                let right_hand = calc_parenthesis_plus(arg,j+1);
                                rhand_2 =  right_hand;
                
                                j = pos_to_skip+1;
                            }
                        }
                        lhand_2 = execute_opeartion('+', lhand_2, rhand_2);
                        rhand_2 = 0;
                        //we executed  once now we need to repeat untill we come across a *
                        let mut next_operator_loc = next_operation(arg, j);
                        let mut next_operator = arg.chars().nth(next_operator_loc).unwrap();
                        //it only returns 0 when no operator could be found
                        // in this case we will continue;
                        if next_operator!= ')'
                        {
                            j = next_operator_loc +1;
                        }
                        else if next_operator == ')'
                        {
                            j = next_operator_loc;
                
                            current_right = lhand_2;
                            i =j;
                            continue;
                        }
                        if next_operator_loc == 0
                        {
                            //if its 0 we know that this is the end
                            //i = arg.chars().count();
                            current_right = lhand_2;
                            break;
                        }
                        //while the next operator is +
                        while next_operator == '+'
                        {
                            c2 = arg.chars().nth(j).unwrap();
                            //use j to detect next number
                            while c2 == ' '
                            {
                                j+=1;
                                c2 = arg.chars().nth(j).unwrap();
                            }
                            //c2 should be a  number
                            let number = match c2.to_string().parse::<u128>()
                            {
                                Ok(number)=> number,
                                Err(_)=> 0,
                            };
                            if number != 0
                            {
                                rhand_2 = number;
                            }
                            if number == 0
                            {
                                if c2 == '('
                                {
                                    //do the parenthesis electric boogaloo
                                    //basically change j to the parenthesis
                                    let pos_to_skip =detect_close_parenthesis(arg, j+1);
                                    let right_hand = calc_parenthesis_plus(arg,j+1);
                    
                                    rhand_2 = right_hand;
                    
                                    j = pos_to_skip+1;
                                }
                                if c2 == ')'
                                {
                                }
                            }
                            lhand_2 = execute_opeartion('+', lhand_2, rhand_2);
                            
                            rhand_2 = 0;

                            next_operator_loc = next_operation(arg, j);
                            next_operator = arg.chars().nth(next_operator_loc).unwrap();
                         
                            if next_operator_loc !=0
                            {
                                j = next_operator_loc +1;
                            }
                            else
                            {
                                j = arg.chars().count();
                            }
                            if next_operator == ')'
                            {
                                j = next_operator_loc;
                            }
                        }
                        current_right = lhand_2;
                        let tmp_result = execute_opeartion('*', current_left, current_right);
                        if tmp_result !=0
                        {
                            result = tmp_result;
                            current_left = tmp_result;
                            current_right = 0;
                        }
                        if j < arg.chars().count()
                        {
                            current_operation = '*';
                        }
                        else if j >= arg.chars().count()
                        {
                            current_operation = ' ';
                        }
                        i = j;
                      
                        continue;
                    }
                    //Evaluate like normal
                    else
                    {
                    let tmp_result =execute_opeartion(current_operation, current_left, current_right);
                    
                        if tmp_result != 0
                        {
                            result = tmp_result;
                            current_left = tmp_result;
                            current_right = 0;
                        }
                        else
                        {

                        }
                   
                    }
                }
                current_operation = '+';

            }
            if c == '*'
            {
                //if a current operation has been set execute it 
                if current_operation != ' '
                {
                    let tmp_result =execute_opeartion(current_operation, current_left, current_right);
                    
                    if tmp_result != 0
                    {
                        result = tmp_result;
                        current_left = tmp_result;
                        current_right = 0;
                    }
                    else
                    {

                    }
                }

                current_operation = '*';
            }
            if c== '('
            {
                //figure out where the next ) is  and calculate the stuff in the parenthesis
                let pos_to_skip =detect_close_parenthesis(arg, i+1);
                let right_hand = calc_parenthesis_plus(arg,i+1);

                if current_left == 0
                {
                    current_left = right_hand;
                }
                else
                {
                    current_right = right_hand;
                }

                i = pos_to_skip+1;
                continue;
            }
            if c== ')'
            {
                if current_operation!= ' ' 
                {
                    //calculate the last operation that we've been given, and then break
                    let res = execute_opeartion(current_operation, current_left, current_right);

                    if res != 0
                    {
                        result = res;
                        current_left = res;
                        current_right = 0;
                        current_operation = ' ';
                        break;
                    }
                    else
                    {
                        break;
                    }
                }
                else
                {
                    //else break
                    break;
                }
            }
        }
        else
        {
            //c is a number
            let number = match c.to_string().parse::<u128>()
            {
                Ok(number)=> number,
                Err(_)=> 0,
            };

            if current_left == 0
            {
                current_left = number;
            }
            else
            {
                current_right = number;
            }
        }
        i +=1;
    }

    if current_operation!= ' ' 
    {
        let res = execute_opeartion(current_operation, current_left, current_right);
        if res != 0
        {
            result = res;
        }
    }

    return result;
}



fn process_part_2(arg: &String)->u128
{
    let mut result = 0;
    let mut my_strings = Vec::new();
    for line in arg.lines()
    {
        my_strings.push(String::from(line));
       
    }

    for st in my_strings
    {
        let  line_result = calc_parenthesis_plus(&st, 0);

        result+= line_result;
    }

    return result;
}
fn process_part_1(arg: &String)->u128
{
    let mut result = 0;
    let mut my_strings = Vec::new();
    for line in arg.lines()
    {
        my_strings.push(String::from(line));
       
    }

    for st in my_strings
    {
        let  line_result = calc_parenthesis(&st, 0);

        result+= line_result;
    }

    return result;
}


pub fn day_18()
{
   /* let test_input = String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    let test_input_1_2 = String::from("6 + ((3 + 6 + 5) * 4) * (4 + 3) * 8 + 6 + 6");
    let test_input_1_3 = String::from("7 + 6 + (9 + 4 * (8 + 7 * 8 + 2 * 7 * 9) * 9 + 5 + 3) + 6 + (7 + 5 + 7 * 9 + 9) * ((4 + 9 * 3 * 4 * 3) + 6 * 8)");
    let test_input_1_5 = String::from("8 * 3 + (2 + 9 + 8 * 5) * 8 * 2 + (4 * 8 * 7 + 5)");
    let test_input_1_4 = String::from("6 + 5 * 3 + 7 + 8 * 2 ");*/
    
   // assert_eq!(process_part_1(&test_input),12240);
    //assert_eq!(process_part_1(&test_input_1_3),3410786448);
   // assert_eq!(process_part_1(&test_input_1_2),3484);
    //assert_eq!(process_part_2(&test_input_1_2),8680);
    //assert_eq!(process_part_2(&test_input_1_4),396);
    //assert_eq!(process_part_2(&test_input),669060);
    //assert_eq!(process_part_2(&test_input_1_3),7920771312);
    //assert_eq!(process_part_2(&test_input_1_5),2420992);
   // println!("yeet");

    let input = utility::get_input_from_filename(&String::from("input/day_18.txt"));

   let result_1= process_part_1(&input);
    let result_2 = process_part_2(&input);
   println!("Day 18 result : {} | {} ",result_1, result_2);
}