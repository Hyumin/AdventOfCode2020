use std::collections::HashMap;

#[path = "utility.rs"]
mod utility;

fn process_part_1(arg: &String) -> u32 
{
    let mut hash_map = HashMap::new();
    
    let  trim_string = arg.replace(&[',','(',')'][..], " ");
    hash_map.insert("examplerino_key", vec![" "," "]);
    for line in trim_string.lines()
     {
        let mut possible_allergans_this_line = Vec::new();
        let mut keys = Vec::new();

        let mut picking_keys = false;
        //
        for word in line.split_whitespace() 
        {
            if word == "contains" 
            {
                picking_keys = true;
                continue;
            }
            if picking_keys 
            {
                keys.push(word);
            }
            else 
            {
                possible_allergans_this_line.push(word);
            }
        }
        for i in 0..keys.len() 
        {
            let key = keys[i].clone();

            if hash_map.contains_key(key)
            {
                //compare differences and push em bck in the 
                let  key_vec = hash_map.get(key).unwrap();
                let mut changed_vec = Vec::new();
                for i in 0.. key_vec.len()
                {
                    let mut iter =possible_allergans_this_line.iter();
                    if iter.find(|&&x| x == key_vec[i]) != None
                    {
                        changed_vec.push(key_vec[i]);
                    }
                }
                hash_map.insert(key, changed_vec.clone());

            } 
            else 
            {
                hash_map.insert(key, possible_allergans_this_line.clone());
            }
        }
    }
    //now loop the input again
    // this time we want to know how many words can't be an allergen
    // first generate a list of all allergans that we found
    // compare every word with ALL alergans and increase the counter each time we can't fidn the word in our list of allergan

    let mut result =0;
    for line in trim_string.lines() {
        for word in line.split_whitespace() 
        {
            if word == "contains" 
            {
                break;
            }
            let mut found = false;
            for val in hash_map.values()
            {
                let mut iter = val.iter();
                //Can this word be the value we wants
                if iter.find(|&&x| x == word) != None
                {
                    found = true;
                    break;
                }
            }
            if found == false
            {
                result +=1;
            }
        }
    }
    return result;
}

pub fn day_21() {
    let sample_input = String::from(
    "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)",
    );

    assert_eq!(process_part_1(&sample_input), 5);

    let puzzle_input = utility::get_input_from_filename(&String::from("input/day_21.txt"));

    let result = process_part_1(&puzzle_input);

    println!("Day 21 result : {}|", result);
}
