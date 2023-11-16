use std::{io, io::Write, collections::HashMap};

pub fn volume_converter() {
    println!("Welcome to the Volume Converter!");

    //Hashmap of metric weight units
    let metric_units: HashMap<&str, f64> = HashMap::from([("cbm", 1.0), ("l", 1000.0), ("ml", 1000000.0)]);
    //Hashmap of imperial weight units
    let imperial_units: HashMap<&str, f64> = HashMap::from([("gal", 1.0), ("qt", 4.0), ("pt", 8.0),
                                                            ("c", 15.773), ("oz", 128.0)]);

    // Same system conversion
    fn same_system(user_input: &(&f64, &String, &String), system: &HashMap<&str, f64>)
                -> f64{
        user_input.0 * (system.get(user_input.2.as_str()).unwrap() /
            system.get(user_input.1.as_str()).unwrap())
    }

    // Different system conversion
        fn system_to_system (user_input: &(&f64, &String, &String), unit_type: &str,
        metric_system: &HashMap<&str, f64>, imperial_system: &HashMap<&str, f64>) -> f64 {
            let mut number_in:f64 = user_input.0.to_owned();
            let cumeter_to_imperial: HashMap<&str, f64> = HashMap::from([("gal", 264.172), ("qt", 1056.69),
                                                    ("pt", 2113.38), ("c", 4166.67), ("oz", 33814.0)]);
            let gallon_to_metric: HashMap<&str, f64> = HashMap::from([("cbm", 0.00378541),
                ("l", 3.78541), ("ml", 3785.41)]);
            if unit_type == "metric" {
                number_in = number_in * metric_system[user_input.1.as_str()];
                return number_in * cumeter_to_imperial.get(user_input.2.as_str()).unwrap();
            }
            else if unit_type == "imperial" {
                number_in = number_in * imperial_system[user_input.1.as_str()];
                return number_in * gallon_to_metric.get(user_input.2.as_str()).unwrap();
            }
            
            0.0
        }


    loop {
        print!("\nPlease enter the number you want to convert (EX: 73.3): ");
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

        // Unit the user wants to convert from
        print!("Please enter the unit you want to convert from (EX: cbm): ");
        io::stdout().flush().expect("Nothing"); // Flushes
        
        let mut unit: String = String::new();
        io::stdin().read_line(&mut unit).expect("Error");
        unit = unit.trim().to_lowercase().to_owned();

        // Unit the user wants to convert to
        print!("Please enter the unit you want to convert to (EX: qt): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        let mut convert_to: String = String::new();
        io::stdin().read_line(&mut convert_to).expect("Error");
        convert_to = convert_to.trim().to_lowercase().to_owned();

        // Checker for the type of unit inputted
        fn unit_type<'a>(input: &String, metric: &HashMap<&str, f64>, imperial: &HashMap<&str, f64>)
                         -> &'a str {
            return if metric.contains_key(input.trim()) {
                "metric"
            } else if imperial.contains_key(input.trim()) {
                "imperial"
            } else {
                "unknown"
            }
        }

        // Check unit given type
        let unit_given_type: &str = unit_type(&unit, &metric_units, &imperial_units);

        // Check convert unit type
        let convert_to_type: &str = unit_type(&convert_to, &metric_units, &imperial_units);

        // Tuple of NUMBER, UNIT, CONVERT TO
        let num_unit_convert: (&f64, &String, &String) = (&filtered_number, &unit, &convert_to);

        // Converting number given to number to convert to | FINAL STEP
        if unit_given_type == "metric" && convert_to_type == "metric" {
            println!("\n{} to {} is {:.3}{}\n", &number_given, &convert_to, same_system(&num_unit_convert,
                                                                    &metric_units), &convert_to);
            break;
        }
        else if unit_given_type == "imperial" && convert_to_type == "imperial" {
            println!("\n{} to {} is {:.3}{}\n", &number_given, &convert_to, same_system(&num_unit_convert,
                                                                    &imperial_units), &convert_to);
            break;
        }
        else if unit_given_type != "unknown" && convert_to_type != "unknown" {
            println!("\n{}{} to {} is {:.3}{}\n", &filtered_number,&unit, &convert_to,
                     system_to_system(&num_unit_convert, &unit_given_type, &metric_units,
                                        &imperial_units), &convert_to);
            break;
        }
        else {
            println!("\nSorry but the units you have entered do not comply. Please try again.")
        }
    }
}