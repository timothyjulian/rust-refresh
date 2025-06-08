struct Student<'a, 'b, T> where T: Ord {
    name: &'a str,
    last_name: &'b str,
    id: T
}

impl<'a, 'b, T> Student <'a, 'b, T> where T: Ord {
    fn longest_name(&self, student: &Student<'a, 'b, T>) ->&'a str{
        if self.name.len() > student.name.len() {
            return self.name;
        }
        student.name
    }
}


fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        return value1;
    }
    value2
}

fn longest_name_student<'a, 'b, T>(student1: &Student<'a, 'b, T>, student2: &Student<'a, 'b, T>) -> &'a str where T: Ord{
    if student1.name.len() > student2.name.len() {
        return student1.name;
    }
    student2.name
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "Timothy";

    let value2 = "Julian";
    let result = longest(value1, value2);
    println!("{}", result);
}

#[test]
fn test__student() {
    let student1 = Student {
        name: "Test",
        last_name: "Hehe",
        id: 32
    };

    let student2 = Student {
        name: "Hohoho",
        last_name: "Hehe123123",
        id: 99
    };
    println!("{}", longest_name_student(&student1, &student2));
    println!("{}", student1.longest_name(&student2));
    println!("{}", student1.id)
}