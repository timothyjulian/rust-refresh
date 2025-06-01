
struct Apple {
    quantity: i32
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_compare() {
    let apple1 = Apple {quantity: 10};
    let apple2 = Apple {quantity: 2};
    println!("Apple 1 == Apple 2 {}", apple1 == apple2);
    println!("Apple 1 > Apple 2 {}", apple1 > apple2);
    println!("Apple 1 < Apple 2 {}", apple1 < apple2);
    println!("Apple 1 >= Apple 2 {}", apple1 >= apple2);
    println!("Apple 1 <= Apple 2 {}", apple1 <= apple2);
}