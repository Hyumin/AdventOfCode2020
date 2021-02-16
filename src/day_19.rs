use std::collections::HashMap;

#[path =  "utility.rs"] 
mod utility;

#[derive(Clone)]
struct Regel  
{
    rule_value : char,
    rules_primary: Vec<i32>,
    rules_or: Vec<i32>,
}

fn check_rule_return_index (line: &String, index: usize, key :i32, rules : &HashMap<i32,Regel>)->i32
{
   // println!("executing rule {}",key);
    if index >= line.chars().count()
    {
        return -1;
    }
    let mut curr_index = index;
    let rule = rules.get(&key).unwrap();
    let mut not_found_primariy =false;
    let prime = rule.rules_primary.clone();
    let or = rule.rules_or.clone();

    if rule.rule_value == 'a'
    {
        //
        if line.chars().nth(curr_index).unwrap() == 'a'
        {
            curr_index +=1;
            return curr_index as i32;
        }
        else
        {
            return -1;
        }
    }
    if rule.rule_value == 'b'
    {
        
        if line.chars().nth(curr_index).unwrap() == 'b'
        {
            curr_index +=1;
            return curr_index as i32;
        }
        else 
        {
           // println!("no b found at {}",curr_index);
            return -1;
        }
    }

    //Check primary rules s
    for k in 0..or.len()
    {
        let v = check_rule_return_index(line, curr_index, or[k], rules);
        if v == -1
        {
            not_found_primariy = true;
            curr_index = index;
            break;
        }
        else 
        {
            curr_index = v as usize;
        }
    }

    if or.len() ==0
    {
        not_found_primariy = true;
    }
    

    if not_found_primariy
    {
        for k in 0..prime.len()
        {
            let v = check_rule_return_index(line, curr_index, prime[k], rules);
            if v == -1
            {
               // println!("{} or failed",key);
                return -1;
            }
            else 
            {
                curr_index = v as usize;
            }
        }   
       // println!("{} prime succeeded",key);
    }
    else 
    {
        //println!("{} or succeeded",key);
    }
  
    return curr_index as i32;
}
fn check_rule_return_index_prime (line: &String, index: usize, key :i32, rules : &HashMap<i32,Regel>)->i32
{
   // println!("executing rule {}",key);
    if index >= line.chars().count()
    {
        return -1;
    }
    let mut curr_index = index;
    let rule = rules.get(&key).unwrap();
    let mut not_found_primariy =false;
    let prime = rule.rules_primary.clone();
    let or = rule.rules_or.clone();

    if rule.rule_value == 'a'
    {
        //
        if line.chars().nth(curr_index).unwrap() == 'a'
        {
            curr_index +=1;
            return curr_index as i32;
        }
        else
        {
            return -1;
        }
    }
    if rule.rule_value == 'b'
    {
        
        if line.chars().nth(curr_index).unwrap() == 'b'
        {
            curr_index +=1;
            return curr_index as i32;
        }
        else 
        {
           // println!("no b found at {}",curr_index);
            return -1;
        }
    }

    //Check primary rules s
    for k in 0..prime.len()
    {
        let v = check_rule_return_index_prime(line, curr_index, prime[k], rules);
        if v == -1
        {
            not_found_primariy = true;
            curr_index = index;
            break;
        }
        else 
        {
            curr_index = v as usize;
        }
    }

    if not_found_primariy
    {
        for k in 0..or.len()
        {
            let v = check_rule_return_index_prime(line, curr_index, or[k], rules);
            if v == -1
            {
               // println!("{} or failed",key);
                return -1;
            }
            else 
            {
                curr_index = v as usize;
            }
        }   
       // println!("{} prime succeeded",key);
    }
    else 
    {
        //println!("{} or succeeded",key);
    }

    return curr_index as i32;
}



fn check_rule_return_index_2 (line: &String, index: usize, key :i32, rules : &HashMap<i32,Regel>)->i32
{
   // println!("executing rule {}",key);
    if index >= line.chars().count()
    {
        return -1;
    }
    let mut curr_index = index as i32;
    let rule = rules.get(&key).unwrap();
    let prime = rule.rules_primary.clone();
    let or = rule.rules_or.clone();

    if rule.rule_value == 'a'
    {
        //
        if line.chars().nth(curr_index as usize).unwrap() == 'a'
        {
            curr_index +=1;
            return curr_index as i32;
        }
        else
        {
            return -1;
        }
    }
    if rule.rule_value == 'b'
    {
        
        if line.chars().nth(curr_index as usize).unwrap() == 'b'
        {
            curr_index +=1;
            return curr_index as i32;
        }
        else 
        {
           // println!("no b found at {}",curr_index);
            return -1;
        }
    }

    //Check primary rules s
    for k in 0..prime.len()
    {
        let v = check_rule_return_index_2(line, curr_index as usize, prime[k], rules);
        if v == -1
        {
            curr_index = -1;
            break;
        }
        else 
        {
            curr_index = v;
        }
    }
    let mut index_2 = index as i32;
    for k in 0..or.len()
    {
        let v = check_rule_return_index_2(line, index_2 as usize, or[k], rules);
        if v == -1
        {
            // println!("{} or failed",key);
            index_2 = -1;
            break;
        }
        else 
        {
            index_2 = v
        }
    }  
    if or.len() == 0
    {
        index_2 = -1;
    }
    
    if index_2 != -1
    {
        return index_2;
    } 
       // println!("{} prime succeeded",key);

    return curr_index as i32;
}

//uber hardcoded function but yolo
fn fortytwo(line: &String, index: usize, rules : & HashMap<i32,Regel>)-> [i32; 4]
{
    let mut result : [i32; 4] = [-1; 4];
    
    for i in 0..4
    {
        result[i] = -1;
    }

    let rule = rules.get(&42).unwrap();

    let prime = rule.rules_primary.clone();
    let or = rule.rules_or.clone();

    let mut index_1 = index as i32;
    let mut index_2 = index as i32;
    let mut index_prim_2 = index as i32;
    let mut index_or_2 = index as i32;

    for i in 0..prime.len()
    {
        let mut res = -1;
        let mut res_2 = -1; 
        if index_1>= 0
        {
           res = check_rule_return_index_prime(line,index_1 as usize, prime[i],rules);
        }

        if index_prim_2 >=0
        {
            res_2 = check_rule_return_index(line, index_prim_2 as usize,prime[i], rules);
        }
        
        if res == -1
        {
            index_1 = -1;
            
        }
        else if res != -1
        {
            index_1 = res;
        }
        if res_2 == -1
        {
            index_prim_2 = -1;
        }
        else if res_2 != -1 
        {
            index_prim_2 = res;
        }
        if res + res_2 == -2
        {
            break;
        }
      
    }

    for i in 0..or.len()
    {
        let mut res = -1;
        let mut res_2 = -1; 
        if index_2>= 0
        {
           res = check_rule_return_index_prime(line,index_2 as usize, or[i],rules);
        }

        if index_or_2 >= 0
        {
            res_2 = check_rule_return_index(line, index_or_2 as usize,or[i], rules);
        }
        
        if res == -1
        {
            index_2 = -1;
        }
        else if res != -1
        {
            index_2 = res;
        }
        if res_2 == -1
        {
            index_or_2 = -1;
        }
        else if res_2 != -1 
        {
            index_or_2 = res;
        }
        if res + res_2 == -2
        {
            break;
        }
      
    }

    result[0] = index_1;
    result[1] = index_prim_2;
    result[2] = index_2;
    result[3] = index_or_2;

    return result;
}
//uber hardcoded function but yolo
fn thirthyone(line: &String, index: usize, rules : & HashMap<i32,Regel>)-> [i32; 4]
{
    let mut result : [i32; 4] = [-1; 4];
    
    for i in 0..4
    {
        result[i] = -1;
    }

    let rule = rules.get(&31).unwrap();

    let prime = rule.rules_primary.clone();
    let or = rule.rules_or.clone();

    let mut index_1 = index as i32;
    let mut index_2 = index as i32;
    let mut index_prim_2 = index as i32;
    let mut index_or_2 = index as i32;

    for i in 0..prime.len()
    {
        let mut res = -1;
        let mut res_2 = -1; 
        if index_1>= 0
        {
           res = check_rule_return_index_prime(line,index_1 as usize, prime[i],rules);
        }

        if index_prim_2 >=0
        {
            res_2 = check_rule_return_index(line, index_prim_2 as usize,prime[i], rules);
        }
        
        if res == -1
        {
            index_1 = -1;
            
        }
        else if res != -1
        {
            index_1 = res;
        }
        if res_2 == -1
        {
            index_prim_2 = -1;
        }
        else if res_2 != -1 
        {
            index_prim_2 = res;
        }
        if res + res_2 == -2
        {
            break;
        }
      
    }

    for i in 0..or.len()
    {
        let mut res = -1;
        let mut res_2 = -1; 
        if index_2>= 0
        {
           res = check_rule_return_index_prime(line,index_2 as usize, or[i],rules);
        }

        if index_or_2 >= 0
        {
            res_2 = check_rule_return_index(line, index_or_2 as usize,or[i], rules);
        }
        
        if res == -1
        {
            index_2 = -1;
        }
        else if res != -1
        {
            index_2 = res;
        }
        if res_2 == -1
        {
            index_or_2 = -1;
        }
        else if res_2 != -1 
        {
            index_or_2 = res;
        }
        if res + res_2 == -2
        {
            break;
        }
      
    }

    result[0] = index_1;
    result[1] = index_prim_2;
    result[2] = index_2;
    result[3] = index_or_2;

    return result;
}

//hardcoded checks rule 0 which consists of rule 8, and rule 11
fn check_rule_brutal_force_part_2(line: &String , rules : &HashMap<i32,Regel>)->i32
{
    let mut return_value = -1;

    let mut cur_index = 0;
    let mut rule_eight_indices = Vec::new();
    rule_eight_indices.push(0);

    // since we're changing rule 11 and 8 to be 42 31 | 42 11 31 and 42 | 42 8 this basically means
    //that we'll have 42 42 31 at the bare minimum or any combination of  2 + x of 42 followed with a 31
    //simply keep calculating 42 untill we can't anymore

    while cur_index != -1
    {
        //get the latest value from the eight_indices vecotr and execute rule 42
        let  index = rule_eight_indices.last().unwrap();
        let res = fortytwo(&line, *index as usize, rules);

        let mut num_minus_ones = 0;
        for i in 0..4
        {
            if res[i] != -1
            {
                rule_eight_indices.push(res[i]);
                if res[i] == 0
                {
                    println!("should not be 0");
                }
            }
            else
            {
                num_minus_ones +=1;
            }
        }
        if num_minus_ones ==4
        {
            break;
        }
    }
    //and storing all these indices inside a vector

    //let mut rule_31_indices = Vec::new();
    //println!("indices found {}", rule_eight_indices.len());
    //then we should execute rule nr 11 starting from all these indices

    let mut victories = vec![0; 0];
    for i in 1..rule_eight_indices.len()
    {
        let mut index : [i32; 4] = [rule_eight_indices[i]; 4];
       
            for j in 0..i
            {
                for w in 0..4
                {
                if index[w] >= 0
                {
                    let res_31 =  thirthyone(&line, index[w] as usize, rules);
          
                    if res_31[w] !=-1
                    {
                        //println!("line {} rule 31 from index {} at {} res = {} line count = {}",line,index,i, res_31,line.chars().count() );
                        //if it is valid see if its the same size as our string>= line.chars().count()
                        if res_31[w] as usize  == (line.chars().count())
                        {
                            //return result cause its valid
                            victories.push(res_31[w]);
                        }
                        else
                        {
                            index[w] = res_31[w];
                        }
                    }
                    else
                    {
                        index[w] = -1;
                    }
             }
            }
        }
    }
    //let mut i =0;
     //then we should execute rule nr 11 starting from all these indices
     /*while true
     {
        if i >= rule_31_indices.len()
        {
            break;
        }

         let index  = rule_31_indices[i];
         let res_31 =  check_rule_return_index(&line, index as usize, 31, rules);
         if res_31 !=-1
         {
             //println!("line {} rule 31 from index {} at {} res = {} line count = {}",line,index,i, res_31,line.chars().count() );
             //if it is valid see if its the same size as our string>= line.chars().count()
             if res_31  == (line.chars().count()) as i32
             {
                 //return result cause its valid
                 return res_31;
             }
             else 
             {
                rule_31_indices.push(res_31)
             }
         }
         i+=1;
         
     } */
     if victories.len()> 0
     {
         return victories[0];
     }

    return return_value;
}
fn process_part_1(input: &String) ->u32
{
    let mut result = 0;
    let mut rules_map = HashMap::new();
    let blank_regel = Regel{rule_value: ' ', rules_primary: Vec::new(),rules_or: Vec::new()};

    let  trim_string = input.replace(&[':','"'][..], " ");
   // println!("{}",input);
    //do some processing
    let mut gather_rules = true;
    for line in trim_string.lines()
    {
        //its empty now we go decipher the rules
        if line == ""
        {
            gather_rules = false;
            continue;
        }

        //Try to decipher a rule
        if gather_rules
        {
            let mut primary = true;
            let mut rule = Regel{rule_value: ' ', rules_primary: Vec::new(),rules_or: Vec::new()};
            let mut key = -1;
            for word in line.split_whitespace()
            {
                let number = match word.to_string().parse::<i32>()
                {
                    Ok(number)=> number,
                    Err(_)=> -1,
                };
                if number != -1
                {
                    if key == -1
                    {
                        key = number;
                        continue;
                    }
                    if primary
                    {
                        rule.rules_primary.push(number);
                    }
                    else
                    {
                        rule.rules_or.push(number);
                    }
               }
               else 
               {
                   if word == "|"
                   {
                       primary= false;
                   }
                   else if word != " "
                   {
                       rule.rule_value = word.chars().nth(0).unwrap();
                   }
               }
            }
            rules_map.insert(key, rule);
        }
        else if ! gather_rules
        {
            //Try to figure out if this string is correct according to rule 0
            /*if check_rule(&line.to_string(), 0, &rules_map)
            {
                println!("line: {} is correct ! GZ",line);
                //result+=1;
            }*/
            let v = check_rule_return_index(&line.trim().to_string(), 0, 0, &rules_map);
            if v != -1
            {
                if v as usize == line.trim().to_string().len()
                {
                    result+=1;
                   // println!("Succes on {} pos {} ",line,v);
                }
                else 
                {  
                  //  println!("legnth failure on {} at pos {} ",line ,v );
                }
            }
            else 
            {
             //   println!("failure on {} ",line);
            }
        }

    }

    return result;
}

fn process_part_2(input: &String) ->u32
{
    let mut result = 0;
    let mut rules_map = HashMap::new();

    let  trim_string = input.replace(&[':','"'][..], " ");
   // println!("{}",input);
    //do some processing
    let mut gather_rules = true;
    for line in trim_string.lines()
    {
        //its empty now we go decipher the rules
        if line == ""
        {
            gather_rules = false;
            //for part two we want to change stuff about rule 11 and rule 8 
            let mut rule_8_new = Regel{rule_value: ' ', rules_primary: Vec::new(),rules_or: Vec::new()};
            rule_8_new.rules_primary.push(42);
            rule_8_new.rules_or.push(42);
            rule_8_new.rules_or.push(8);

            let mut rule_11_new= Regel{rule_value: ' ', rules_primary: Vec::new(),rules_or: Vec::new()};
            rule_11_new.rules_primary.push(42);
            rule_11_new.rules_primary.push(31);

            rule_11_new.rules_or.push(42);
            rule_11_new.rules_or.push(11);
            rule_11_new.rules_or.push(31);

            rules_map.insert(8, rule_8_new);
            rules_map.insert(11, rule_11_new);
            continue;
        }

        //Try to decipher a rule
        if gather_rules
        {
            let mut primary = true;
            let mut rule = Regel{rule_value: ' ', rules_primary: Vec::new(),rules_or: Vec::new()};
            let mut key = -1;
            for word in line.split_whitespace()
            {
                let number = match word.to_string().parse::<i32>()
                {
                    Ok(number)=> number,
                    Err(_)=> -1,
                };
                if number != -1
                {
                    if key == -1
                    {
                        key = number;
                        continue;
                    }
                    if primary
                    {
                        rule.rules_primary.push(number);
                    }
                    else
                    {
                        rule.rules_or.push(number);
                    }
               }
               else 
               {
                   if word == "|"
                   {
                       primary= false;
                   }
                   else if word != " "
                   {
                       rule.rule_value = word.chars().nth(0).unwrap();
                   }
               }
            }
            rules_map.insert(key, rule);
        }
        else if ! gather_rules
        {
            //Try to figure out if this string is correct according to rule 0
            /*if check_rule(&line.to_string(), 0, &rules_map)
            {
                println!("line: {} is correct ! GZ",line);
                //result+=1;
            }*/
            /*let v = check_rule_return_index_2(&line.trim().to_string(), 0, 0, &rules_map);
            if v != -1
            {
                if v as usize == line.trim().to_string().len()
                {
                    result+=1;
                    println!("Succes on {} pos {} ",line,v);
                }
                else 
                {  
                    println!("legnth failure on {} at pos {} ",line ,v );
                }
            }
            else 
            {
                println!("failure on {} ",line);
            }*/
            let v2 = check_rule_brutal_force_part_2(&line.trim().to_string(), &rules_map);
            if v2 != -1
            {
               // println!("line {} succes on {} ",line ,v2);
                result+=1;
            }

            }

    }

    /*for keyss in rules_map.keys()
    {
        let rule = rules_map.get(&keyss).unwrap();

        let mut string_prim= String::from("");
        let mut string_or = String::from("");

        for num in rule.rules_primary.clone()
        {
            string_prim.push_str(&num.to_string());
            string_prim.push(',');
        }
        for num in rule.rules_or.clone()
        {
            string_or.push_str(&num.to_string());
            string_or.push(',');
        }

        println!("Rule nr: {} with value {} , prim: {} and or: {}",keyss,rule.rule_value,string_prim,string_or);

    } */

    return result;
}

pub fn day_19()
{
    let sample_input = String::from(r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
aabbb
aaaabba"#);

let test_2 = String::from(r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#);

  // assert_eq!(process_part_1(&sample_input),2);
  // assert_eq!(process_part_1(&test_2),3);
 //  assert_eq!(process_part_2(&test_2),12);
    let input = utility::get_input_from_filename(&String::from("input/day_19.txt"));
    let test_input_22 = utility::get_input_from_filename(&String::from("input/day_19_test.txt"));
    //assert_eq!(process_part_2(&test_input_22),126);

    let result_1 = process_part_1(&input);
    let result_2 = process_part_2(&input);
    println!("Day 19 result = {} | {}",result_1,result_2);
}