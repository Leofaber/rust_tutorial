fn main() {
    
    // if statement

    let number = 6;
    
    if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 3 or 2");
    }

    // if statement is an expression

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);


    // loop statement is an expression

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);    


    // for with iterator

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // for with Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");


}