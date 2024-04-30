fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner scope {x}");
    }
    println!("out scope {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("len spaces is {spaces}");

}
