fn main() {
    let user1 = User {
        active: true,
        username: String::from("frank"),
        email: String::from("example@xx.com"),
        sign_in_count: 1
    };
    println!("user1: {:?}", user1);
    // user1: User { active: true, username: "frank", email: "example@xx.com", sign_in_count: 1 }

    println!("user1 name: {}", user1.username); // user1 name: frank

    let username = "John".to_string();
    let mut user2 = User {
        username,
        ..user1
    };
    user2.sign_in_count = 2;
    println!("user2: {:?}", user2);
    // user2: User { active: true, username: "John", email: "example@xx.com", sign_in_count: 2 }
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
