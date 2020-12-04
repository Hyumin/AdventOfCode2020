

//Reads the lines from the terminal and return after entering done identifier
pub fn get_input( done_iden : &String) -> &String
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

    return &inputStrings;
}