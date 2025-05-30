use crate::struct_type::Person;

pub struct SimplePerson{
    pub name: String
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello_string(&self) -> String;
    fn say_hello(&self, name: &str);
    fn say_hello_to(&self, name: &str) -> String;
}

pub trait CanSayGoodbye {
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}

// super trait where it inherits the methods
trait CanSay: CanSayHello + CanSayGoodbye {
    fn say(&self) {
        println!("{}", self.hello());
        println!("{}", self.say_goodbye());
    }
}

impl CanSayHello for Person {
    fn say_hello_string(&self) -> String {
        format!("Hello my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello, {} my name is {}", name, self.first_name)
    }
    
    fn say_hello(&self, name: &str) {
        println!("Hello, {}, i', from trait", name);
    }
}

impl CanSayGoodbye for Person {
    fn say_goodbye(&self) -> String {
        format!("Goodbye from {}", self.first_name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {} from {}", name, self.first_name)
    }
}

impl CanSayGoodbye for SimplePerson {
    fn say_goodbye(&self) -> String {
        format!("Goodbye from {}", self.name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {} from {}", name, self.name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello_string());
}

fn say_goodbye_and_hello(value: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", value.say_hello_string());
    println!("{}", value.say_goodbye());
}

fn create_simple_person(name: String) -> impl CanSayGoodbye {
    SimplePerson {name}
}


#[test]
fn trait_test() {
    let person = Person {
        first_name: String::from("Timo"),
        last_name: String::from("Jul"),
        age: 12
    };

    println!("{}", person.say_hello_to("Eko"));
    println!("{}", person.say_hello_string());
    println!("{}", person.hello());

    // for overriding method
    CanSayHello::say_hello(&person, "wkwk");    
    Person::say_hello(&person, "wkwkw");


    say_hello_trait(&person);
    say_goodbye_and_hello(&person);
}

#[test]
fn test_impl_trait(){
    let simple_person = create_simple_person(String::from("timo"));
    println!("{}", simple_person.say_goodbye());
}