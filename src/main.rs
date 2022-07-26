use std::io;

fn main() {
    println!("Temperature Converter!!");

    loop {
        println!("1. Convert to Celcius");
        println!("2. Convert to Fahrenheit");
        println!("Type 1 or 2 to start...");
        println!("Press any letter to stop");

        let mut option = String::new(); 

        io::stdin()
             .read_line(&mut option)
             .expect("Failed to read line");

        // println!("{:?}", option);

        if option.trim() == "1" {
            convert_to_celcius();
        }
        else if option.trim() == "2" {
            convert_to_fahrenheit(); 
        }
        else {
            break;
        }
    }
}

fn convert_to_celcius() {
    println!("Enter Celcius");

    let mut celcius = String::new(); 

    io::stdin()
        .read_line(&mut celcius)
        .expect("Failed to read line");

    let celcius: i32 = celcius.trim().parse().expect("Please type a number"); 

    let fahrenheit = celcius + 32;

    println!("{celcius} Celcius is equal to {fahrenheit} Fahrenheit"); 
    println!("\n");
}

fn convert_to_fahrenheit() {
    println!("Enter Fahrenheit");

    let mut fahrenheit = String::new(); 

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: i32 = fahrenheit.trim().parse().expect("Please type a number"); 

    let celcius = fahrenheit - 32;

    println!("{fahrenheit} Fahrenheit is equal to {celcius} Celcius");
    println!("\n");
}
