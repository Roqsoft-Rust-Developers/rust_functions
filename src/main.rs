use std::io;

fn main() {
    perform_operation();
    temperature_converter();

}

fn perform_operation() {
    let mut number_a = String::new();
    let mut number_b = String::new();
    let mut operator = String::new();

    println!("Please enter a number: ");

    io::stdin().read_line(&mut number_a).expect("No value provided");

    let number_a: f64 = number_a
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    println!("Please enter an operator: (*, /, +, -)");

    io::stdin().read_line(&mut operator).expect("No value provided");

    let operator = operator.trim();

    println!("Please enter another number:");

    io::stdin().read_line(&mut number_b).expect("No value provided");

    let number_b: f64 = number_b
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    if operator == "+" {
        let sum: f64 = number_a + number_b;
        let rounded_sum = format!("{:.2}", sum);
        println!("{} plus {} is equal to {}", number_a, number_b, rounded_sum)
    } else if operator == "-" {
        let sub: f64 = number_a - number_b;
        println!("{} minus {} is equal to {}", number_a, number_b, sub)
    } else if operator == "*" {
        let mul: f64 = number_a * number_b;
        println!("{} times {} is equal to {}", number_a, number_b, mul)
    } else if operator == "/" {
        if number_b != 0.0 {
            let div: f64 = number_a / number_b;
            println!("{} divided by {} is equal to {:.2}", number_a, number_b, div)
        } else {
            println!("Sorry cannot divide by zero")
        }
    } else {
        println!("Sorry the operator you provided does not exist")
    }
}


fn temperature_converter() {
    // C = (F - 32) * 5/9
    // F = (C * 9/5) + 32

    let mut temp_value = String::new();

    let mut unit = String::new();

    println!("Please enter a temperature value");

    io::stdin().read_line(&mut temp_value).expect("No value provided");

    let temp_value: f64 = temp_value.trim().parse().expect("Please input a valid number");

    println!("Please enter temperature unit");

    io::stdin().read_line(&mut unit).expect("No value provided");

    let unit = unit.trim().to_lowercase();

    if unit == "c" || unit == "celsius" {
        let val: f64 = (temp_value * (9.0 / 5.0)) + 32.0 ;
        println!("{}°F", val)
    } else if unit == "f" || unit == "fahrenheit" {
        let val: f64 = (temp_value - 32.0) * (5.0 / 9.0);
        println!("{}°C", val)
    } else {
        println!("The unit provided is not recognized");
    }
}
