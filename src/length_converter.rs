use std::{collections::HashMap, io, io::Write};

pub fn _length_converter() {
    loop{
        print!("Please enter the number then the unit (lowercase and abbreviated) after \
        (EX: 14mm): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        // Number the user types with it's unit
        let mut number_given: String = String::new();
        io::stdin().read_line(&mut number_given).expect("Error");
        number_given = number_given.trim().to_owned();

        // Creates a Vector to allow the user number to be iterated over
        let num_given_vec: Vec<char> = number_given.chars().collect();

        // Alphabet in a String
        let letters: String = String::from("abcdefghijklmnopqrstuvwxyz");

        // This section creates the unit variable and makes it store the unit the user gives
        let unit: String;
        if letters.contains(num_given_vec[num_given_vec.clone().len() -2].clone()) {
            unit = format!("{}{}", num_given_vec[num_given_vec.clone().len() -2].clone(),
                           num_given_vec[num_given_vec.clone().len() -1].clone());
        }
        else {
            unit = format!("{}", num_given_vec[num_given_vec.clone().len() -1].clone());
        }

        // THIS IS THE USER NUMBER WITHOUT THE UNIT | VERY IMPORTANT
        let number: f64 = (number_given.replace(&unit, "")).parse::<f64>().unwrap();

        print!("Enter the unit you want to convert to (EX: in): ");
        io::stdout().flush().expect("Nothing"); // Flushes

        // Unit the user wants to convert to
        let mut convert_to: String = String::new();
        io::stdin().read_line(&mut convert_to).expect("Error");
        convert_to = convert_to.trim().to_owned();

        // Hash map for metric units and imperial units
        let metric_units: HashMap<&str, f64> = HashMap::from([("km", 1000.0), ("m", 1.0),
            ("cm", 0.01), ("mm", 0.001)]);
        let imperial_units: HashMap<&str, f64> = HashMap::from([("mi", 63_360.0), ("yd", 36.0),
            ("ft", 12.0), ("in", 1.0)]);

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

        // Metric conversion
        fn metric(number_in: &f64, unit_in: &String, unit_to: &String, metric: &HashMap<&str, f64>)
                  -> f64{
            number_in * (metric.get(unit_in.as_str()).unwrap() /
                metric.get(unit_to.as_str()).unwrap())
        }

        // Imperial conversion
        fn imperial(number_in: &f64, unit_in: &String, unit_to: &String,
                    imperial: &HashMap<&str, f64>) -> f64 {
            number_in * (imperial.get(unit_in.trim()).unwrap() /
                imperial.get(unit_to.trim()).unwrap())
        }

        // Metric to Imperial or vice versa
        fn system_to_system (number_in: &f64, unit_in: &String, unit_to: &String, unit_type: &str, metric_system: &HashMap<&str, f64>, imperial_system: &HashMap<&str, f64>)
                             -> f64 {
            let mut number_in:f64 = number_in.to_owned();
            let meter_to_imperial: HashMap<&str, f64> = HashMap::from([("mi", 1609.34), ("yd", 1.094),
                ("ft", 3.281), ("in", 39.37)]);
            let inch_to_metric: HashMap<&str, f64> = HashMap::from([("km", (1.0 / 39370.0)),
                ("m", (1.0 / 39.37)), ("cm", 2.54), ("mm", 25.4)]);
            if unit_type == "metric" {
                number_in = number_in * metric_system[unit_in.as_str()];
                return number_in * meter_to_imperial.get(unit_to.as_str()).unwrap();
            }
            else if unit_type == "imperial" {
                number_in = number_in * imperial_system[unit_in.as_str()];
                return number_in * inch_to_metric.get(unit_to.as_str()).unwrap();
            }
            else {
                0.0
            }
        }

        // Converting number given to number to convert to | FINAL STEP
        if unit_given_type == "metric" && convert_to_type == "metric" {
            println!("{} to {} is {:.3}{}", &number_given, &convert_to, metric(&number,
                                                                               &unit, &convert_to, &metric_units), &convert_to);
            break;
        }
        else if unit_given_type == "imperial" && convert_to_type == "imperial" {
            println!("{} to {} is {:.3}{}", &number_given, &convert_to, imperial(&number,
                                                                                 &unit, &convert_to, &imperial_units), &convert_to);
            break;
        }
        else if unit_given_type != "unknown" && convert_to_type != "unknown" {
            println!("{} to {} is {:.3}{}", &number_given, &convert_to,
                     system_to_system(&number, &unit, &convert_to,
                                      &unit_given_type, &metric_units, &imperial_units), &convert_to);
            break;
        }
        else {
            println!("Sorry but the units you have entered do not comply. Please try again.")
        }
    }
}