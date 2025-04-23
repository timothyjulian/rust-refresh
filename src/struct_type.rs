pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

pub struct GeoPoint(pub f64, pub f64);

fn print_person(person: &Person) {
    println!("{}", person.age);
    println!("{}", person.first_name);
    println!("{}", person.last_name);
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, I'm {}", name, self.first_name);
    }
}

impl GeoPoint {
    pub fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Timothy");
    let person = Person {
        first_name,
        last_name: String::from("Julian"),
        age: 24,
    };

    print_person(&person);

    let person2 = Person { age: 99, ..person };

    print_person(&person2);
}

#[test]
fn test_struct_tuple() {
    let geo = GeoPoint(12.0, 12.0);
    println!("{}", geo.0);
    println!("{}", geo.1);
}

#[test]
fn test_person_method() {
    let person = Person {
        first_name: String::from("Timothy"),
        last_name: String::from("Julian"),
        age: 23,
    };

    person.say_hello("Uiiaa");
}

#[test]
fn test_geopoint_ass_function() {
    let geo_point = GeoPoint::new(35.6764, 139.6500);

    println!("{} {}", geo_point.0, geo_point.1);
}
