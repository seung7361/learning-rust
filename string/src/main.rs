fn main() {
    let x: i32 = 5;
    let y: i32 = x;

    println!("x: {}", x);

    let s1: String = String::from("Hello");
    let s2 = s1.clone();

    println!("s1: {}", s1);
}