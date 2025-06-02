use std::ops::Add;

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_add() {
    let apple_1 = Apple { quantity: 1 };

    let apple_2 = Apple { quantity: 2 };

    // let apple_sum = apple_1.add(apple_2);
    // println!("{}", apple_sum.quantity);
    let apple_sum = apple_1 + apple_2;
    println!("{}", apple_sum.quantity);
}
