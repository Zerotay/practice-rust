fn main() {
    let a = [1,2,3,4,5];
    let b = [1..6];
    //println!("{a}, {b}");
    let slice = &a[1..3];
    let slice_2 = &b[1..3];
    assert_eq!(slice, &[2,3]);
    //assert_eq!(slice_2, &[2,3]);
}
