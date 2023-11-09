use super::converters::{length_converter::length_converter, temperature_converter::temperature_converter};
use std::{io, io::Write};

#[allow(dead_code)]
pub fn unit_converter() {
    println!("\nWelcome to the Unit Converter!\nAvailable converters:\n");

    // This section prints the available converters
    let converters_available = Vec::from(["length_converter", "temperature_converter",
                    "weight_converter (NOT AVAILABLE)", "volume_converter (NOT AVAILABLE)"]);
    for i in 0..converters_available.len() {
        println!("\t{}: {}", i + 1, converters_available[i]);
        if i == converters_available.len() - 1 {
            println!();
        }
    }

    // This section creates a vector of functions
    let converter_functions: Vec<fn()> = Vec::from([length_converter, temperature_converter]);

    println!("Please enter the number of the converter you want to use: ");

    //Variable for when converter finishes
    let mut converter_finished: bool = false;

    loop {
        if converter_finished {
            print!("Would you like to use another converter? (y/n): ");
            io::stdout().flush().expect("Nothing"); // Flushes

            let mut another_converter: String = String::new();
            io::stdin().read_line(&mut another_converter).expect("Error");
            another_converter = another_converter.trim().to_owned();

            if another_converter == "y" {
                converter_finished = false;
                println!("\nAvailable converters:\n");
                for i in 0..converters_available.len() {
                    println!("\t{}: {}", i + 1, converters_available[i]);
                    if i == converters_available.len() - 1 {
                        println!();
                    }
                }
            }
            else if another_converter == "n" {
                break;
            }
            else {
                println!("Sorry but the answer you have entered is not valid. Please try again.");
                continue;
            }
        }
        
        print!("Answer (type \"exit\") to quit: ");
        io::stdout().flush().expect("Nothing"); // Flushes

        let mut converter_number: String = String::new();
        io::stdin().read_line(&mut converter_number).expect("Error");
        converter_number = converter_number.trim().to_owned();

        // This section checks if the number given is a number and only one digit
        if Vec::from(["exit", "Exit", "EXIT"]).contains(&converter_number.as_str()){
            break;
        }
        else if !String::from("0123456789").contains(converter_number.chars().next().unwrap()){
            println!("Sorry but the number you have entered is not a number. Please try again.");
            continue;
        }
        else if converter_number.trim().parse::<i32>().unwrap() > converter_functions.len() as i32 {
            println!("Sorry but the number you have entered is not a valid converter. Please try again.");
            continue;
        }
        else {
            let converter_number = converter_number.trim().parse::<i32>().unwrap();
            println!();
            converter_functions[(converter_number -1) as usize]();
            converter_finished = true;
        }
    }
}