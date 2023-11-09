use std::{collections::HashMap, io, io::Write};

#[allow(dead_code)]
pub fn length_converter() {
    println!("Welcome to the Length Converter!");
    loop{
        
        print!("\nPlease enter the number you want to convert (EX: 5): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        // Number the user types
        let mut number_given: String = String::new();
        io::stdin().read_line(&mut number_given).expect("Error");
        number_given = number_given.trim().to_owned();

        // This section checks if the number given is a number
        if !String::from("0123456789").contains(number_given.chars().next().unwrap()) {
            println!("\nSorry but the number you have entered is not a number. Please try again.");
            continue;
        }


        // Hash map for metric units and imperial units
        let metric_units: HashMap<&str, f64> = HashMap::from([("km", 1000.0), ("m", 1.0),
            ("cm", 0.01), ("mm", 0.001)]);
        let imperial_units: HashMap<&str, f64> = HashMap::from([("mi", 63_360.0), ("yd", 36.0),
            ("ft", 12.0), ("in", 1.0)]);


        // This section creates the unit variable and makes it store the unit the user gives
        print!("\nEnter the unit you want to convert from (EX: mm): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        let mut unit: String = String::new();
        io::stdin().read_line(&mut unit).expect("Error");
        unit = unit.trim().to_owned();

        // This section checks if the unit given is a unit
        if !metric_units.contains_key(unit.trim()) && !imperial_units.contains_key(unit.trim()) {
            println!("\nSorry but the unit you have entered is not a unit. Please try again.");
            continue;
        }
        

        // THIS IS THE USER NUMBER WITHOUT THE UNIT | VERY IMPORTANT
        let number: f64 = number_given.parse::<f64>().unwrap();

        print!("\nEnter the unit you want to convert to (EX: in): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        // Unit the user wants to convert to
        let mut convert_to: String = String::new();
        io::stdin().read_line(&mut convert_to).expect("Error");
        convert_to = convert_to.trim().to_owned();
        

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
        let unit_given_type: &str = unit_type(&unit, &metric_units,
                                              &imperial_units);

        // Check convert unit type
        let convert_to_type: &str = unit_type(&convert_to, &metric_units,
                                              &imperial_units);
        
        // Tuple of NUMBER, UNIT, CONVERT TO
        let num_unit_convert: (&f64, &String, &String) = (&number, &unit, &convert_to);

        // Same system conversion
        fn same_system(user_input: &(&f64, &String, &String), system: &HashMap<&str, f64>)
                  -> f64{
            user_input.0 * (system.get(user_input.1.as_str()).unwrap() /
                system.get(user_input.2.as_str()).unwrap())
        }


        // Different system conversion
        fn system_to_system (user_input: &(&f64, &String, &String), unit_type: &str,
        metric_system: &HashMap<&str, f64>, imperial_system: &HashMap<&str, f64>) -> f64 {
            let mut number_in:f64 = user_input.0.to_owned();
            let meter_to_imperial: HashMap<&str, f64> = HashMap::from([("mi", 1609.34), ("yd", 1.094),
                ("ft", 3.281), ("in", 39.37)]);
            let inch_to_metric: HashMap<&str, f64> = HashMap::from([("km", (1.0 / 39370.0)),
                ("m", (1.0 / 39.37)), ("cm", 2.54), ("mm", 25.4)]);
            if unit_type == "metric" {
                number_in = number_in * metric_system[user_input.1.as_str()];
                return number_in * meter_to_imperial.get(user_input.2.as_str()).unwrap();
            }
            else if unit_type == "imperial" {
                number_in = number_in * imperial_system[user_input.1.as_str()];
                return number_in * inch_to_metric.get(user_input.2.as_str()).unwrap();
            }
            
            0.0
        }

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
            println!("\n{}{} to {} is {:.3}{}\n", &number,&unit, &convert_to,
                     system_to_system(&num_unit_convert, &unit_given_type, &metric_units,
                                        &imperial_units), &convert_to);
            break;
        }
        else {
            println!("\nSorry but the units you have entered do not comply. Please try again.")
        }
    }
}