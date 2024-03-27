struct User {
    name: String,
    is_retired: bool,
    age: u32,
}

fn main() {
    let mut felipe = User {
        name: "Felipe".to_string(),
        is_retired: false,
        age: 32,
    };

    let user2 = build_user(String::from("Felipe"), 32);

    let user3 = User {
        name: "Alberto".to_string(),
        ..felipe
    };

    felipe.name = "Isabela".to_string();
}

fn build_user(name: String, age: u32) -> User {
    User {
        name,
        age,
        is_retired: false,
    }
}
