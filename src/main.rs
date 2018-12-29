fn main() {
    let x: String = "a".to_owned();
    let y: Option<String> = Some(x);
    println!("{:?}", y);
    let t: &String = y.as_ref().unwrap();
    println!("{}", t);
    println!("{:?}", y);
    let x: &String = y.as_ref().unwrap();
    println!("{}", x);

    println!("Hello, world!");
}
