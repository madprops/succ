use crate::
{   
    s, pp
};

use std::
{
    io::{self, Write}
};

// Centralized function to handle user input
pub fn get_input(message: &str) -> String
{
    pp!(format!("{}: ", message));
    io::stdout().flush().unwrap();

    let mut input = s!();

    match io::stdin().read_line(&mut input) 
    {
        Ok(_) => input,
        Err(_) => s!()
    }
}

// Asks the user for a yes/no answer
pub fn ask_bool(message: &str, critical:bool) -> bool
{
    let prompt = if critical {" (Y, n)"} else {" (y, n)"};

    loop
    {
        let ans = get_input(&[message, prompt].concat());

        match ans.trim()
        {
            "y" => 
            {
                if !critical {return true}
            },
            "Y" => return true,
            "n" | "N" => return false,
            "" => return false,
            _ => {}
        }
    }
}

#[allow(dead_code)]
// Asks the user to input a string
pub fn ask_string(message: &str, trim: bool) -> String
{
    let ans = get_input(message);
    if trim {s!(ans.trim())} else {ans}
}