use std::io::stdin;

fn main() {

    let t: (i32, f64, u8) = (500, 6.4, 1);
    
    // destructuring
    let (x,y,z) = t;
    
    // index access
    let first = t.0;
    let second = t.1;
    let third = t.2;


    let a = [1,2,3,4,5];
    let c : [i32; 5] = [1,2,3,4,5];
    let b = [3; 5]; // b=[3,3,3,3,3]

    let first = a[0];
    
    let mut index = String::new();
    stdin()
        .read_line(&mut index)
        .expect("");

    let index: usize = index
        .trim()
        .parse()
        .expect("");

    // this operation will panic 
    // at runtime
    let panic = a[index]; 
}
