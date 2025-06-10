use std::collections::{BTreeSet, HashSet};

#[test]
fn test_hash_set() {
    let mut hash_set: HashSet<i32> = HashSet::new();
    hash_set.insert(1);
    hash_set.insert(1);
    hash_set.insert(2);
    hash_set.insert(2);
    hash_set.insert(3);
    hash_set.insert(3);

    for num in &hash_set {
        println!("{}", num);
    }
}

#[test]
fn test_btree_set() {
    let mut btree_set: BTreeSet<i32> = BTreeSet::new();
    btree_set.insert(2);
    btree_set.insert(2);
    btree_set.insert(3);
    btree_set.insert(3);
    btree_set.insert(1);
    btree_set.insert(1);

    for num in &btree_set {
        println!("{}", num);
    }
}
