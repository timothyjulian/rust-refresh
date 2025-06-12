macro_rules! hi {
    () => {
        println!("Hi!");
    };
    ($name: expr) => {
        println!("Hi, {}!", $name);
    };
}

macro_rules! iterate {
    ($array: expr) => {
        for i in $array {
            println!("{}", i);
        }
    };
    ($($item: expr), *) => {
        $(
            println!("{}", $item);
        )*
    };
}

#[test]
fn test_macro() {
    hi!();
    hi!("Timo");
    hi! {
        "Timo"
    }
}

#[test]
fn test_iterate() {
    iterate!([1, 2, 3, 4, 5]);
    iterate!(1, 2, 3, 4, 5);
}
