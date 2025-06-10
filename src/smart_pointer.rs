fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

fn print_category(category: &ProductCategory) {
    println!("{:?}", category);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End,
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