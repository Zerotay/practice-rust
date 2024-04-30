fn main() {
    println!("This is integer types");
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111;
    let byte = b'a';

    println!("decimal is {decimal}");
    println!("hex is {hex}");
    println!("octal is {octal}");
    println!("binary is {binary}");
    println!("byte is {byte}");
    println!();

    println!("This is float types");
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x is {x}, y is {y}");
    println!();

    println!("This is calculation");
    let a = 65.55 - 33.;
    let b = 33.3 / 1.2;
    let c = -3 / 4;
    let d = 12 % 3;
    println!("{a}, {b}, {c}, {d}");
    println!();



}
