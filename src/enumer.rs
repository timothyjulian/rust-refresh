use crate::struct_type::GeoPoint;
use crate::struct_type::Person;

enum Level {
    Regular,
    Premium,
    Platinum,
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::EWallet(wallet, number) => {
                println!("Paying with e wallet {} {} {}", wallet, number, amount);
            }
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} {}", bank, number, amount);
            }
        }
    }
}

#[test]
fn test_enum() {
    let _level = Level::Platinum;
}

#[test]
fn test_payment_pattern() {
    let _payment = Payment::BankTransfer(String::from("BCA"), String::from("123123"));
    _payment.pay(9999);
}


#[test]
fn test_pattern_value() {
    let val = String::from("TEST");

    match val.as_str() {
        "TEST" | "PROD" => {
            println!("Testing aja ye kan")
        }
        other => {
            println!("ndataw {}", other);
        }
    }
}

#[test]
fn test_range_pattern() {
    let val = 100;

    match val {
        75..=100 => {
            println!("kkm ga seh");
        }
        50..75 => {
            println!("yo wat");
        }
        _ => {
            println!("o aja ye kan");
        }
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint::new(2.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long : {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long : {}, lat : {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("Timothy "),
        last_name: String::from("Julian"),
        age: 20,
    };

    match person {
        Person { first_name, last_name, .. } => {
            println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(2.0, 1.0);

    match point {
        GeoPoint(long, _) => {
            println!("long : {}", long);
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 9;

    let result = match value {
        0 => "nol",
        1 => {
            "satu"
        }
        2 => {
            "dua"
        }
        _ => "invalid"
    };

    println!("{}", result);
}