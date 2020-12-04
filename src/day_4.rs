use std::io;

fn verifypw_2(arg: &String)->bool
{
    let mut result = false;

   let string_without_double_punt = arg.replace(&[':'][..], " ");
    let mut result_counter = 0;

    let valids = ["byr","iyr","eyr","hgt","hcl","ecl","pid" ];
    let mut iter =string_without_double_punt.split_whitespace();

    let mut stupid_bools = [false,false,false,false,false,false,false];

    for word in string_without_double_punt.split_whitespace()
    {

        if stupid_bools[0]
        {
            let birthyear : i32 = match word.parse()
            {
                Ok(birthyear)=>birthyear,
                Err(_) =>  continue,
            };
            
            if birthyear >= 1920 && birthyear <= 2002
            { 
                result_counter +=1;
            }
        }
        if stupid_bools[1] 
        {
            let num : i32 = match word.parse()
            {
                Ok(num)=>num,
                Err(_) => continue ,
            };
            
            if num >= 2010 && num <= 2020
            { 
                result_counter +=1;
            }
        }
        if stupid_bools[2]
        {
            let num : i32 = match word.parse()
            {
                Ok(num)=>num,
                Err(_) => continue,
            };
            
            if num >= 2020 && num <= 2030
            { 
                result_counter +=1;
            }
        }
        if stupid_bools[3]
        {
            //check last 2 letters if its in or cm,
            //then parse everything to the left of it to a number
            // and see if its between the values on the site

            //might need to be word.len()-3 .. wordlen()-1
            let _legnth = word.chars().count() as usize;
            let _measurement = &word[_legnth-2.._legnth];
            let _val = &word[0.._legnth-2];
            let num : i32 = match _val.parse()
            {
                Ok(num)=>num,
                Err(_) =>  continue,
            };

            if _measurement == "cm"
            {
                if num>=150&& num <=193
                {
                    result_counter += 1;
                    println!("cms correct");
                }
            }
            else if _measurement =="in"
            {
                if num>=59&& num <=76
                {
                    result_counter += 1;
                    println!("in correct");
                }
            }
        }
        if stupid_bools[4]
        {
            //Check first letter, see if it starts with a #
            // and check if its followed by six characters
            //check whether those characters are 0-9 or a-f
           if(word.chars().count()==7)
           {
                if Some('#')  == word.chars().nth(0 as usize)
                {
                    let _hexboi = &word[1..7];
                    let mut _valid_counter = 0;
                    for c in _hexboi.chars()
                    {

                    let num = c.to_digit(16).unwrap();
                       if num >= 0x0&& num<= 0xf 
                       {    
                         _valid_counter +=1;
                       }
                    }
                    if(_valid_counter == 6)
                    {
                        result_counter +=1;
                    }
                }
            }
        }
        if stupid_bools[5]
        {
            if word == "amb"|| word == "blu"||word == "brn"||word == "gry"||word == "hzl" ||word == "oth"||word== "grn"
            {
                result_counter+=1;
            }
        }
        if stupid_bools[6]
        {
            if word.chars().count() ==9
            {
                //if we can't convert to a number we'll continue anyway
                let num : i32 = match word.parse()
                {
                    Ok(num)=>num,
                    Err(_) =>  continue,
                };
                result_counter+=1;

            }
        }

         for i in 0..stupid_bools.len()
         {
            if word == valids[i]
            {
               stupid_bools[i] = true;
            }
            else
            {
              stupid_bools[i] =false;
            }
         }
    }
       
   if result_counter>= valids.len()
   {
       result = true;
       println!("wow its correct!");
   }


    return result;
}
fn verifypw_1(arg: &String)->bool
{
    let mut result = false;

   let string_without_double_punt = arg.replace(&[':'][..], " ");
    let mut result_counter = 0;

    let valids = ["byr","iyr","eyr","hgt","hcl","ecl","pid" ];
   for word in string_without_double_punt.split_whitespace()
   {
       for i in 0..valids.len()
       {
            if word == valids[i]
            {
                result_counter +=1;
            }
        }
   }
   if result_counter>= valids.len()
   {
       result = true;
   }


    return result;
}

fn part_2(arg: &String) ->i32
{

 let mut result = 0;

    let mut pw_object= String::new();
    let mut linecounter = 0;
    //Record every line untill we find an empty line
    //once empty line is found check if everything is in order
    // and reset string after
    for line in arg.lines()
    {
        linecounter +=1;
        if line.chars().count() == 0 as usize
        {
            if verifypw_2(&pw_object)
            {
                result+=1;
            }
            pw_object= String::from("");
        }
        pw_object += line;
        pw_object += " ";
    }
    //If the end of file does not have a whitespace
    //we need to execute the check once more
    // if it does the pw_object will be empty anyways and result in false
    if verifypw_2(&pw_object)
    {
        result+=1;
    }
    println!("lines {}",linecounter);
    return result;
}
fn part_1( arg: &String) -> i32
{
    let mut result = 0;

    let mut pw_object= String::new();
    let mut linecounter = 0;
    //Record every line untill we find an empty line
    //once empty line is found check if everything is in order
    // and reset string after
    for line in arg.lines()
    {
        linecounter +=1;
        if line.chars().count() == 0 as usize
        {
            if verifypw_1(&pw_object)
            {
                result+=1;
            }
            pw_object= String::from("");
        }
        pw_object += line;
        pw_object += " ";
    }
    //If the end of file does not have a whitespace
    //we need to execute the check once more
    // if it does the pw_object will be empty anyways and result in false
    if verifypw_1(&pw_object)
    {
        result+=1;
    }
    println!("lines {}",linecounter);
    return result;
}
//Reads the lines from the terminal and return after entering done identifier
pub fn get_input( done_iden : &String) -> String
{
    let mut inputStrings = String::new();
    let  doneIdentifier = String::from(done_iden);
  
    //Type custom input in the terminal to see how the function behaves with different input
    println!("Custom input, Please enter input in the terminal to create a string to iterate through. When done type {}",doneIdentifier);
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
  
      inputStrings+= text.trim();
      inputStrings+= "\n";
    }

    return inputStrings;
}
pub fn day_4()
{

    println!("Day 4");
    let done_identifier = String::from("done");

    let default_input =String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in");

    part_1(&default_input);

    assert_eq!(part_1(&default_input),2);


    let invalid = String::from("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007");
//assert_eq!(part_2(&invalid),0);

let valid = String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");

    assert_eq!(part_2(&valid),4);
    let input = get_input(&done_identifier);
    println!( "{}", part_2(&input));


}