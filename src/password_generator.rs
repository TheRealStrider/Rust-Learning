use std::{io, io::Write};
use rand::Rng;

pub fn _password_generator() {
    print!("Type in the length you want the password to be (EX: 5): ");
    io::stdout().flush().expect("Nothing");

    // Length the password needs to be
    let mut password_length: String = String::new();
    io::stdin().read_line(&mut password_length).expect("Failed");
    let password_length: u32 = password_length.trim().parse::<u32>().unwrap();

    // Whether to include special characters
    print!("Do you want to include special characters? (EX: !@#): ");
    io::stdout().flush().expect("Nothing");
    let mut include_special: String = String::new();
    io::stdin().read_line(&mut include_special).expect("Failed");
    include_special = include_special.trim().to_lowercase().to_string();


    let alphabet_lower: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz").chars().collect();
    let alphabet_upper: Vec<char> = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ").chars().collect();
    let numbers: Vec<char> = String::from("0123456789").chars().collect();
    let special_characters: Vec<char> = String::from("?!@#$%&").chars().collect();
    let mut password: String = String::new();


    for _i in 0..password_length {
        let choose_type: u32 = rand::thread_rng().gen_range(0..4);
        let x: char = alphabet_lower[rand::thread_rng().gen_range(0..26)];
        let y: char = alphabet_upper[rand::thread_rng().gen_range(0..26)];
        let z: char = numbers[rand::thread_rng().gen_range(0..10)];
        let s: char = special_characters[rand::thread_rng().gen_range(0..7)];

        if include_special == "yes" {
            match choose_type {
                0 => password += &x.to_string(),
                1 => password += &y.to_string(),
                2 => password += &z.to_string(),
                3 => password += &s.to_string(),
                _ => {}
            }
        }
        else {
            match choose_type {
                0 => password += &x.to_string(),
                1 => password += &y.to_string(),
                2 => password += &z.to_string(),
                3 => password += &x.to_string(),
                _ => {}
            }
        }
    }
    println!("Your password is {}", password);

}