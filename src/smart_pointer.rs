use std::{ops::Deref, rc::Rc};

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End,
}

struct MyValue<T> {
    value: T,
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
enum Brand {
    Of(String, Rc<Brand>),
    End,
}

fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

fn print_category(category: &ProductCategory) {
    println!("{:?}", category);
}

fn say_hello(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn test_box() {
    let value = Box::new(5);
    println!("{}", value);

    display_number(*value);
    display_number_reference(&value);
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
        "Laptop".to_string(),
        Box::new(ProductCategory::Of(
            "Dell".to_string(),
            Box::new(ProductCategory::End),
        )),
    );

    print_category(&category);
}

#[test]
fn test_dereference() {
    let value1 = Box::new(10);
    let value2 = Box::new(3);

    let result = *value1 * *value2;
    println!("{}", result)
}

#[test]
fn test_dereference_struct() {
    let val = MyValue {
        value: 9
    };

    println!("{}", *val);
}

#[test]
fn test_dereference_coercion() {
    let name = MyValue {
        value: "String".to_string()
    };
    say_hello(&name);
}

#[test]
fn test_multiple_ownership() {
    let apple = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    let laptop = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    let smartphone = Brand::Of("Smartphone".to_string(), Rc::clone(&apple));

    println!("{:?}", apple);
    println!("{:?}", smartphone);
    println!("{:?}", laptop);
    println!("Apple reference count: {}", Rc::strong_count(&apple));

}