/* 構造体 */
struct User {
    name: String,
    age: u32,
    email: String,
    uid: String,
    active: bool,
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        name: String::from("user1"),
        age: 32,
        email: String::from("user1@umayuma.com"),
        uid: String::from("501783454357801"),
        active: true,
    };

    user1.email = String::from("user1@explorer.com");

    let user2 = init_User("user2".to_string(),
                            22,
                            "user2@explorer.com".to_string(),
                            "598273450".to_string());

    let user3 = User {
        name: String::from("user3"),
        email: String::from("user3@exnample.cum"),
        uid: String("27345872489752"),
        ..user2,  // user2と同じ箇所はこれでまとめて更新可能
    };

    /* タプル構造体 */
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);

    
}

fn init_User(username: String, age: u32, email: String, id: String) -> User {
    User {
        name: username,
        age,  // フィールド名と変数名が同じなら省略可能
        email,
        uid: id,
        active: true,
    }
}
