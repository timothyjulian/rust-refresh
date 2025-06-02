fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val * 2),
    }
}

#[test]
fn test_double() {
    let result = double(Some(5));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);

    if let Some(x) = double(Some(7)) {
        println!("{}", x);
    } else {
        println!("Nothing")
    }
}
