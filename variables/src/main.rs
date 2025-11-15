// 変数，関数，文・式，コメント，制御フローに関する勉強

fn main() {
    println!("Hello, world!");

    // let x = 5;  unmutable
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINT: u32 = 100_000;  // 定数
    println!("const: {}", MAX_POINT);

    /* shadowing */
    let y = 5;
    let y = y + 1;
    {
        let y = y*2;
        println!("The value of y is in this scope: {}", y);
    }
    println!("The value of y is out of scope: {}", y);

    /* 四則演算 */
    // addition
    let sum = 5 + 10;

    // subtraction
    let diff = 95.5 - 2.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.1;
    let floored  = 2 / 3;  // floored = 0

    //remainder
    let remainder = 43 % 8;

    println!("sum: {}\ndiff: {}\nmulti: {}\nquot: {}\nfloored: {}\nremainder: {}",
                sum, diff, product, quotient, floored, remainder);
    
    /* boolean */
    let t = true;
    let f = false;

    /* char */
    let c = 'c';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("char: {}\nz: {}\ncat: {}", c, z, heart_eyed_cat);

    /* 複合型 */
    /* タプル型 */
    let tup: (i32, f64, u8) = (500, 2.3, 1);
    let (ta, tb, tc) = tup;

    println!("The value of tb is {}", tb);
    println!("The value of tup.2 is {}", tup.2);

    /* リスト型 */
    let list = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"];
    let list5: [i32; 5] = [1, 2, 3, 4, 5];  // 変数名: [型; 要素数] = ...
    let list_new = [3; 5];  // let a = [3, 3, 3, 3, 3];と同値

    println!("list[0]: {}", list[0]);
    println!("fist month is {}", months[0]);

    // 配列要素への無効なアクセス
    // list5を使う
    println!("Please enter an aaray index.");
    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Faild to read line");

    let index: usize = index
                        .trim()
                        .parse()
                        .expect("Index entered was not a number");
    
    let element = list5[index];
    println!("The value of list5[{}] is: {}", index, element);

    /* 関数 */
    another_fuinction(x, c);
}

fn another_fuinction(x: i32, uint_label: char) {
    println!("=========================");
    println!("Calling another function!");
    println!("Arguments are: {}{}", x, uint_label);
    // {}で新しいスコープを作成するブロック
    {
        let x = 3;
        x + 1;
    }

    // 文: 値を返さない
    let y = 6;
    // 式: 何かしらの値を返す
    // 例: 四則演算
    let z = {
        let a = 8;
        a + 1  // ;を付けてしまうと文になってしまいzに値を返さなくなってしまう
    };

    println!("The most simple returning 6 function: {}", six());
    println!("func func: {}", XpOne(six()));
    
    println!("=========================");
}

fn six() -> i32 {
    6  // ;をつけると式ではなく文になってしまう
}

fn XpOne(x: i32) -> i32 {
    x + 1
}

// コメント

// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
// ここで何か複雑なことをしていて、長すぎるから複数行のコメントが必要なんだ。
// ふう！願わくば、このコメントで何が起きているか説明されていると嬉しい。

