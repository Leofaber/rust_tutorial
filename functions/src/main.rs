fn main() {
    let x = plus_one(5);
    void(x);
    let y = plus_one(5);
    let t = as_tuple(x, y);
    println!("x={} y={}",t.0,t.1);
}
 
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn void(x: i32) {
    println!("The value of x is: {}", x);    
}

fn as_tuple(x: i32, y: i32) -> (i32, i32) {
    let t = (x, y);
    t
}