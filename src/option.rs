pub fn option() {
    println!("{:?}", None.map(|x: &str| x));
    println!("{:?}", Some("1").map(|x: &str| x));
}
