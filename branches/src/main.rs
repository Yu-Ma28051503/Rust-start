// 制御フロー

fn main() {
    println!("Hello, world!");

    let number = 3;  // true
    // let number = 7;  // false

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // ifはboolしか評価しない
    // if number {
    //     println!("conpile error");
    // }

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 5 == 0 {
        println!("divisible by 5");
    } else if number % 6 == 0 {
        println!("divisible by 6");
    } else if number % 7 == 0 {
        println!("divisible by 7");
    } else {
        println!("not divisible 5~7");
    }

    // ifは式なのでlet文の右辺に持ってくることができる
    let t = true;
    let f = false;
    let condition = if t { 5 } else { 6 };
    // let num = if f { 7 } else { "eight" };  // 同じ型じゃないとエラー

    println!("{}: Since if is an expression, it can be placed on the right-hand side of a let statement.", condition);
}
