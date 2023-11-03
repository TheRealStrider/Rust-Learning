use std::{io, io::Write};
use rand::Rng;

#[macro_export]
macro_rules! random {
    ($x: expr, $y: expr) => {
        rand::thread_rng().gen_range($x..$y)
    };
}


pub fn _password_generator() {
    print!("Type in the length you want the password to be (EX: 5): ");
    io::stdout().flush().expect("Nothing");

    // Length the password needs to be
    let mut password_length: String = String::new();
    io::stdin().read_line(&mut password_length).expect("Failed");
    let password_length: usize = password_length.trim().parse::<usize>().unwrap();

    // Whether to include special characters
    print!("Do you want to include special characters? (y/n) (EX: !@#): ");
    io::stdout().flush().expect("Nothing");

    let mut include_special: String = String::new();
    io::stdin().read_line(&mut include_special).expect("Failed");
    include_special = include_special.trim().to_lowercase().to_string();


    let alphabet_lower: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz").chars().collect();
    let alphabet_upper: Vec<char> = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ").chars().collect();
    let numbers: Vec<char> = String::from("0123456789").chars().collect();
    let special_characters: Vec<char> = String::from("?!@#$%&").chars().collect();
    let mut password: String = String::new();


    while password.len() < password_length {
        let choose_type: u32 = random!(0, 4);
        let x: char = alphabet_lower[random!(0, 26)];
        let y: char = alphabet_upper[random!(0, 26)];
        let z: char = numbers[random!(0, 10)];
        let s: char = special_characters[random!(0, 7)];

        if choose_type == 0 {
            password.push(x);
        } else if choose_type == 1 {
            password.push(y);
        } else if choose_type == 2 {
            password.push(z);
        } else if choose_type == 3{
            if include_special == "y" {
                password.push(s);
            } else {
                password.push(x);
            }
        }
    }
    println!("Your password is {}", password);

}