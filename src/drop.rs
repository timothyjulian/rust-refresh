struct Book {
    title: String
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Clearing memory for {}", self.title);
    }
}

#[test]
fn test_drop_cleanup() {
    let book = Book {
        title: "Harry Potter".to_string()
    };

    println!("{}", book.title);
}