use std::fmt::{Debug, Display};

struct Category {
    id: String,
    name: String,
}

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.id, self.name)
    }
}

#[test]
fn debug_struct() {
    let cat = Category {
        id: String::from("9"),
        name: String::from("Manga"),
    };
    println!("{:?}", cat);
    println!("{}", cat);
}
