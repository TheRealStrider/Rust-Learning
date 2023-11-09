use std::{io, io::Write};

pub fn temperature_converter() {
    println!("Welcome to the Temperature Converter!");
    loop{
        print!("\nPlease enter the number you want to convert (EX: 5): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        // Number the user types
        let mut number_given: String = String::new();
        io::stdin().read_line(&mut number_given).expect("Error");
        number_given = number_given.trim().to_owned();

        //Vector of number given
        let mut number_given_vector: Vec<char> = number_given.chars().collect();
        number_given_vector.retain(|i| *i != ' ' && *i != ',');

        // This section checks if the number given is a number
        let mut check_for_decimal: bool = false;
        let mut second_decimal: bool = false;
        let mut check_for_letters: bool = false;
        for i in 0..number_given_vector.len() {
            if number_given_vector[0] == '.' {
                check_for_decimal = true;
                number_given_vector.insert(0, '0');
            }
            else if !String::from("0123456789.").contains(number_given_vector[i]) {
                check_for_letters = true;
                break;
            }
            else if number_given_vector[i] == '.' {
                if check_for_decimal {
                    second_decimal = true;
                    break;
                }
                else {
                    check_for_decimal = true;
                    continue;
                }
            }
        }
        
        // Final check //
        if check_for_letters {
            println!("\nSorry but the number you have entered is not a number. Please try again.");
            continue;
        }
        else if second_decimal {
            println!("Only one decimal point is allowed. Please try again.");
            continue;
        }

        // Takes filtered number and assigns to original number
        number_given = number_given_vector.into_iter().collect();
        let filtered_number: f64 = number_given.parse::<f64>().unwrap();
        
        // This section creates the unit variable
        print!("\nEnter the unit you want to convert from (EX: C): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        let mut unit: String = String::new();
        io::stdin().read_line(&mut unit).expect("Error");
        unit = unit.trim().to_uppercase().to_owned();

        // This section checks if the unit given is a unit
        if !String::from("CFK").contains(unit.chars().next().unwrap()) && unit.len() == 1 {
            println!("\nSorry but the unit you have entered is not a unit. Please try again.");
            continue;
        }

        // This section creates the unit variable to convert to
        print!("\nEnter the unit you want to convert to (EX: F): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        let mut convert_to: String = String::new();
        io::stdin().read_line(&mut convert_to).expect("Error");
        convert_to = convert_to.trim().to_uppercase().to_owned();

        // This section checks if the unit given is a unit
        if !String::from("CFK").contains(convert_to.chars().next().unwrap()) && convert_to.len() == 1 {
            println!("\nSorry but the unit you have entered is not a unit. Please try again.");
            continue;
        }

        // This section checks if the unit given is the same as the unit the user wants to convert to
        if unit == convert_to {
            println!("\nSorry but the unit you have entered is the same as the unit you want to convert to. Please try again.");
            continue;
        }

        // Conversion
        let mut final_number: f64 = 0.0;

        // Celsius to Fahrenheit
        if unit == "C" && convert_to == "F" {
            final_number = filtered_number * 1.8 + 32.0;
        }

        // Celsius to Kelvin
        else if unit == "C" && convert_to == "K" {
            final_number = filtered_number + 273.15;
        }

        // Fahrenheit to Celsius
        else if unit == "F" && convert_to == "C" {
            final_number = (filtered_number - 32.0) / 1.8;
        }

        // Fahrenheit to Kelvin
        else if unit == "F" && convert_to == "K" {
            final_number = (filtered_number + 459.67) * (5.0 / 9.0);
        }

        // Kelvin to Celsius
        else if unit == "K" && convert_to == "C" {
            final_number = filtered_number - 273.15;
        }

        // Kelvin to Fahrenheit
        else if unit == "K" && convert_to == "F" {
            final_number = filtered_number * (9.0 / 5.0) - 459.67;
        }

        // Prints final number
        println!("\n{}{} = {:.2}{}\n", filtered_number, unit, final_number, convert_to);
        break;
    }
}