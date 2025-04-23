struct Person{
    first_name: String,
    last_name: String,
    age: u8
}

struct GeoPoint(f64, f64);

fn print_person(person: &Person) {
    println!("{}", person.age);
    println!("{}", person.first_name);
    println!("{}", person.last_name);
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Timothy");
    let person = Person {
        first_name,
        last_name: String::from("Julian"),
        age: 24
    };

    print_person(&person);

    let person2 = Person {
        age: 99,
        ..person
    };

    print_person(&person2);
}

#[test]
fn test_struct_tuple(){
    let geo = GeoPoint(12.0, 12.0);
    println!("{}", geo.0);
    println!("{}", geo.1);
}