use std::io::stdin;
use std::process;

fn get_input(message: &String) -> String {
    let stdin = stdin();
    let mut x = String::new();
    println!("{}",message);
    match stdin.read_line(&mut x) {
        Ok(_n) => {
            // println!("You inserted: {}. Bytes: {}",x.trim(), n);
            return x
        },
        Err(err) => {
            println!("Error: {}", err);
            return err.to_string()
        }
    };
}

fn get_f_input(message: &String) -> f32 {
    
    let input = get_input(message);
    let input : f32 = input
                        .trim()
                        .parse()
                        .expect("Expected f32");
    input

}

fn get_s_input(message: &String) -> String {
    String::from(get_input(message).trim())
}


fn celsius_to_fahrenheit(cel: f32) -> f32 {
    (cel * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(far: f32) -> f32 {
    (far - 32.0) * 5.0/9.0
}

fn main() {

    let temp : f32;
    let new_temp: f32;
    let mut um : String;
    let new_um : String;
    
    temp = get_f_input(& String::from("Insert the temperature"));
    
    loop {
        um = get_s_input(& String::from("Insert the unit of measure (C or F)"));
        if um == "C" || um == "F" {
            break;
        }
    }

    if um == "C" {
        new_um = String::from("Fahrenheit");
        new_temp = celsius_to_fahrenheit(temp);
    }
    else if um == "F" {
        new_um = String::from("Celsius");
        new_temp = fahrenheit_to_celsius(temp);
    }
    else {
        println!("Something went wrong..um={}",um);
        process::exit(1);
    }

    println!("{} {} = {} {}", temp, um, new_temp, new_um);
    

}
