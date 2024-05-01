fn main() {
    let s1 = String::from("hello");
    let len = cal_len(&s1);
    println!("The length of '{}' is {}.", s1, len);
    println!();

    let mut s2 = String::from("hello");
    modify_val(&mut s2);
    println!("modfied.. {s2}");
    println!();

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    let r3 = &mut s;
    println!("{r3}");

}

fn cal_len(s: &String) -> usize {
    s.len()
}

fn modify_val(s: &mut String) {
    s.push_str(", world");
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}
