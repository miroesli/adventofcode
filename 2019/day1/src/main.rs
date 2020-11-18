use std::fs::File;
use std::io::{BufRead, BufReader};

const DEBUG: bool = false;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for (_index, weight) in reader.lines().enumerate() {
        if DEBUG {
            println!("{}. {:?}", _index + 1, weight);
        }
        let weight: std::string::String = match weight {
            Ok(weight) => weight,
            Err(_) => continue,
        };
        let weight: i32 = match weight.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let total_fuel_for_module = calculate_required_fuel(weight / 3 - 2);
        total = total + total_fuel_for_module;
        if DEBUG {
            println!("{} -> {}", weight, total_fuel_for_module);
        }
    }
    println!("Total Fuel Required is: {}", total);
}

fn calculate_required_fuel(fuel: i32) -> i32 {
    if fuel <= 0 {
        return 0;
    }
    return fuel + calculate_required_fuel(fuel / 3 - 2);
}
