#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_debug() {
    let company = Company {
        name: "Rust".to_string(),
        location: "USA".to_string(),
        website: "https://www.rust-lang.org".to_string(),
    };

    println!("{:?}", company);

    let company2 = company.clone();

    println!("{}", company == company2);
}