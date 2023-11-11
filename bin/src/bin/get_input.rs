use std::{io::{self, Write}, str::FromStr};

fn main() {
   let num:i32 = get_input("Enter something");
   println!("{num}");
   

 
}

fn get_input<T:FromStr>(message: impl Into<String>)
->T{
    let mut input = String::new();
    print!("{}: ",message.into());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
     match input.trim().parse::<T>(){
        Ok(input) => input,
        Err(_) => todo!(),
    }
    
}
