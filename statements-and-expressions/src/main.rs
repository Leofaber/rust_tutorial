fn main() {

    // the last line of this expression cannot have a semicolumn
    // otherwise it will turn into a statement that does not 
    // return any value...
    let x = {
        let y = 10;
        y + 1
    };

    println!("x={}",x);
}
