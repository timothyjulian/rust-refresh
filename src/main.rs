mod closure;
mod comparing;
mod enumer;
mod format;
mod generic;
mod maps;
mod options;
mod overload_operator;
mod ownership;
mod recursive;
mod sequences;
mod slice;
mod struct_type;
mod traits;
mod sets;

fn main() {}

#[test]
fn hello_test() {
    println!("Hello world!");
}

#[test]
fn variable_test() {
    let name = "Timothy Julian";
    println!("Hello 123 {}", name);
}

#[test]
fn explicit_test() {
    let number: i32 = 50;
    println!("{}", number);
}

#[test]
fn tuple() {
    let test = (1, 'a', 4.5);
    println!("{:?}", test);
    println!("{} | {} | {}", test.0, test.1, test.2);
    let (a, _, b) = test;
    println!("{} <> {}", a, b);
}

fn unit() {
    println!("Hello World!");
}

#[test]
fn unit_test() {
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    println!("{} {}", array[0], array[3]);
    println!("{}", array.len());
}

#[test]
fn twod_array() {
    let array_2d = [[1, 2], [3, 4]];

    println!("{:?}", array_2d);
    println!("{} {}", array_2d[0][1], array_2d[0][0]);
}

#[test]
fn heap() {
    let heap_data = String::from("value");
    let stack_data = 123;

    println!("{} {}", heap_data, stack_data);
}

#[test]
fn string_slice() {
    let name = "   Timothy Julian      ";
    let trim = name.trim();

    println!("{}|{}", name, trim);
}

#[test]
fn string_type() {
    let mut name = String::from("Timothy Julian");
    name.push_str(" DANA");
    println!("{}", name);

    let atmojo = name.replace("Julian", "Atmojo");
    println!("{}", atmojo);
}

#[test]
fn data_copy_ownership() {
    let a = 10;
    let mut b = a; //copy data

    b = 9;

    println!("{a} {b}"); // a still 10, b changed to 9
}

#[test]
fn if_let_test() {
    let value = 9;
    let result = if value >= 8 { "Good" } else { "Hehe" };

    println!("{}", result);
}

#[test]
fn loop_test() {
    let mut value = -5;
    'test: loop {
        println!("{value}");
        if value >= 0 {
            break 'test;
        }
        value += 1;
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 5 {
            break counter * 5;
        }
    };

    println!("{result}");
}

#[test]
fn range() {
    let range = 0..5;
    range.for_each(|x| println!("{x}"));
}

#[test]
fn range_inclusive() {
    let range = 0..=5;
    range.for_each(|x| println!("{x}"));
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_function() {
    say_goodbye("Eko", "Khannedy");
    say_goodbye("Budi", "Nugraha");
    say_goodbye("Joko", "Susilo");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result: i32 = factorial_loop(5);
    println!("{}", result);

    let result: i32 = factorial_loop(-10);
    println!("{}", result);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Timothy Julian");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Timothy", "Paulus"));
    println!("{}", s.contains("Julian"));
    println!("{}", s.starts_with("Tim"));
    println!("{}", s.ends_with("Julian"));
    println!("{}", s.trim());
    println!("{}", &s[0..3]);
    println!("{:?}", s.get(0..3));
}
