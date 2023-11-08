use std::{io, io::Write};
use yard;

#[allow(dead_code)]
pub fn calculator(){
    print!("Welcome to the calculator! Start by entering numbers and operators (EX: 5 + 5). \
    To exit, type 'exit': ");
    io::stdout().flush().expect("Nothing"); // Flushes

    //Alphabet
    let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    //Numbers
    let numbers: String = String::from("0123456789");

    // Previous result
    let mut previous_result: f64 = 0.0;

    // Result
    let mut result: f64;

    //Calculator
    loop {
        // Get input
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        input = input.trim().to_string();

        //Check whether to exit
        if &input == "exit" {
           break;
        }


        // Vector to be used for the calculation
        let mut input_vec: Vec<char> = Vec::new();

        // Check whether the input is valid
        for i in input.chars() {
            if alphabet.contains(i) {    
                println!("Please enter a valid number");
                break;
            }
            else if numbers.contains(i) {
                input_vec.push(i);
            }
            else if (i == '(') && numbers.contains(input_vec[&input_vec.len() -1]) {
                input_vec.push('*');
                input_vec.push(i);
            }
            else if i == ')'{
                input_vec.push(i);
            }
            else if (i == '+') || (i == '-') || (i == '*') || (i == '/') {
                input_vec.push(i);
            }
            else if i == ' ' {
                continue;
            }
            else {
                println!("Please enter a valid number");
                break;
            }
            
        }
        
        // Convert the vector to a string
        let option = input_vec.iter().map(|c| c.to_string()).collect::<String>();
        //DEBUG
        //println!("Option: {}", option);
        
        // Check whether to output the previous result or not
        if !numbers.contains(input_vec[0]) {
            result = yard::evaluate(format!("{}{}", previous_result, option).as_str()).unwrap();
        }
        else {
            result = yard::evaluate(&option).unwrap();
        }

        // Set the previous result to the current result
        previous_result = result;

        // Output the result
        println!("Result: {}", result);
    }
}