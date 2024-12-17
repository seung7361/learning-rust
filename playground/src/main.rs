fn main() {
    let a = Box::new(1);
    drop(&a);

    println!("{a}");
}