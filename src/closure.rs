#[test]
fn closure() {
    let sum = |value1: i32, value2: i32| -> i32 { value1 + value2 };

    println!("{}", sum(1, 1));
}

fn print_map(value: &String, map_fn: fn(&String) -> String) {
    println!("{}", map_fn(value));
}

fn to_uppercase(value: &String) -> String {
    value.to_uppercase()
}

#[test]
fn test_print_map() {
    let name = String::from("Timothy Julian");
    print_map(&name, |value| value.to_uppercase());
    print_map(&name, to_uppercase);
}
