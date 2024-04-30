use std::io;
fn main() {
    println!("This is example of tuple");
    let tup = (400, 2.1, 'd');
    let (x,y,z) = tup;
    println!("{x}, {y}, {z}");
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!();



    println!("This is example of array!\nPlz enter ann array index");
    let a = [1,2,3,4,5];
    let a = [4;5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    
    println!("The value of the element at index {index} is : {element}");

}
