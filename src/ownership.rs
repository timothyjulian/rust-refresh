#[test]
fn transfer_ownership() {
    let name1 = String::from("Timothy");
    let name2 = name1; // ownership moved here

    // println!("{name1}"); // error here
    println!("{name2}");

    let name_ori = String::from("DANA");
    let name_clone = name_ori.clone();

    println!("{name_ori} {name_clone}");
}

fn print_ownership(name: String) {
    print!("{}", name);
}

#[test]
fn test_ownership() {
    let name = String::from("Tim");
    print_ownership(name); // takes ownership
                           // println!("{}", name); // error here
}

fn return_ownership(first_name: String, second_name: String) -> String {
    format!("{} {}", first_name, second_name)
}

#[test]
fn test_return_ownership() {
    let name = return_ownership(String::from("Tim"), String::from("Jul")); // the value return owned by this function

    println!("{}", name);
}

fn return_borrow(first_name: &String, second_name: &String) -> String {
    // first_name.push_str("haha"); // by default reference is immutable, even though the owner is mutable
    format!("{} {}", first_name, second_name)
}

#[test]
fn test_return_ownership_with_ref() {
    let mut first_name = String::from("Tim");
    let name = return_borrow(&first_name, &String::from("Jul")); // reference passed

    println!("{}", name);
}

fn mutable_reference(first_name: &mut String) {
    first_name.push_str("test");
}

#[test]
fn test_mutable_reference() {
    let mut first_name = String::from("hahahaha");
    let mut second_name = String::from("wokwokw");

    // if value is current borrowed, it can't be reassigned
    // let ptr = &second_name;
    // second_name = String::from("asdasd");
    // println!("{}", ptr);

    let mut test = &mut first_name;
    mutable_reference(test);
    println!("{}", test);

    test = &mut second_name;
    println!("{}", test);

    println!("{}", first_name);
}
