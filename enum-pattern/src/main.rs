/* enum */
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4addr {}
struct Ipv6addr {}

enum IpAddr_struct {
    V4(Ipv4addr),
    V6(Ipv6addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}

// enumもメゾットを定義できる
impl Message {
    fn call(&self) {}
}

// Tは10章で詳しくやる
// ジェネリック型引数
// どんな型でもとる
// 標準stdで定義されている
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    println!("Hello, world!");
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    let lov4addr = IpAddr::V4(127, 0, 0, 1);
    let lov6addr = IpAddr::V6(String::from("::1"));

    route(ipv4);
    route(ipv6);

    let msg = Message::Write(String::from("hello"));
    msg.call();

    let some_number = Some(5);
    let some_string = Some("some");
    let absent_number: Option<i32> = None;

    // 型の力
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // println!("x + y = {}", x+y);  // えらー
}

fn route(ipv: IpAddrKind) {}
