#[test]
fn slice_reference() {
    let array = [1,2,3,4,5,6,7,8,9,10];
    let slice1 = &array[..];
    println!("{:?}", slice1);

    let slice2 = &array[1..5];
    println!("{:?}", slice2);

    let slice3 = slice2; // since reference is pointer and stored in stack, it is copied
    println!("{:?}", slice2);
    println!("{:?}", slice3);
}

#[test]
fn string_slice(){
    let name = String::from("Timothy Julian");
    let first_name = &name[0..7];
    println!("{}", first_name);

    let second_name = &name[8..];
    println!("{}", second_name);
}