use crate::traits::{CanSayGoodbye, SimplePerson};

// i32 is default
struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        &self.x
    }
}

struct Hi<T: CanSayGoodbye> {
    value: T,
}

enum Value<T> {
    NONE,
    VALUE(T),
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> { x: 1, y: 2 };

    let float = Point::<f64> { x: 1.4, y: 2.5 };

    println!("{} {}", integer.x, integer.y);
    println!("{} {}", float.x, float.y);
}

#[test]
fn test_generic_enum() {
    let enum_generic = Value::<i32>::VALUE(9);
    match enum_generic {
        Value::NONE => println!("None"),
        Value::VALUE(value) => println!("{}", value),
    }
}

#[test]
fn test_generic_struct_with_trait() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("value"),
        },
    };
    println!("{}", hi.value.say_goodbye());
}

#[test]
fn test_generic_function() {
    let result = min::<i32>(10, 20);
    println!("{}", result);
}

#[test]
fn test_generic_mehthod() {
    let point = Point { x: 1, y: 2 };
    println!("{} {}", point.get_x(), point.get_y());
    println!("{}", point.get_value());
}
