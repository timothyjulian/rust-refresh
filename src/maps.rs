use std::collections::{BTreeMap, HashMap};

#[test]
fn test_hash_map() {
    let mut hash_map = HashMap::<i32, String>::new();
    hash_map.insert(5, String::from("test"));
    hash_map.insert(8, String::from("haha"));

    println!(
        "{}\n{}",
        hash_map.get(&5).unwrap(),
        hash_map.get(&8).unwrap()
    );
}

#[test]
fn test_btree_map() {
    let mut btree_map = BTreeMap::<i32, String>::new();
    btree_map.insert(1, String::from("test"));
    btree_map.insert(3, String::from("hihi"));
    btree_map.insert(2, String::from("haha"));

    for (key, value) in &btree_map {
        println!("{} {}", key, value);
    }

    println!("======");

    btree_map
        .iter()
        .for_each(|tup| println!("{} {}", tup.0, tup.1));

    println!("======");

    btree_map
        .iter()
        .for_each(|(key, value)| println!("{} {}", key, value));
}
