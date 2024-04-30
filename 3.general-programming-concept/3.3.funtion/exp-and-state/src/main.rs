fn main() {
    let _y = 7;// this never returns any value;


    let a =  {
        let x = 3;
        x + 1
    };
    println!("a is {a}");

    let b = five();
    println!("b is {b}");
}

fn five()-> i32{
    6
}
