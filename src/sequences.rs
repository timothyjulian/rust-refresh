use std::collections::{LinkedList, VecDeque};

#[test]
fn test_vec() {
    let mut names = Vec::<String>::new();
    names.push(String::from("Timothy"));
    names.push(String::from("Julian"));
    names.push(String::from("Ahihihi"));

    for name in &names {
        println!("{name}");
    }

    println!("{:?}", names);
}

#[test]
fn test_vec_deque() {
    let mut nums = VecDeque::<i32>::new();
    nums.push_back(1);
    nums.push_back(2);
    nums.push_front(3);
    nums.push_front(4);

    nums.iter().for_each(|num| println!("{num}"));
}

#[test]
fn test_linked_list() {
    let mut nums = LinkedList::<i32>::new();
    nums.push_back(9);
    nums.push_back(8);
    nums.push_front(7);
    nums.push_front(6);

    let sum: i32 = nums.iter().sum();
    println!("{sum}");

    nums.iter().map(|x| 1 + x).for_each(|num| print!("{num} "));
}
