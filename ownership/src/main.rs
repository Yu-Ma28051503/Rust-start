fn main() {
    println!("Hello, world!");
    // let s_stack = String::new();
    let mut s_heap = String::from("hello");

    println!("s_heap: {}", s_heap);

    s_heap.push_str(", world!");
    println!("s_heap: {}", s_heap);

    let s2_heap = s_heap;

    // ポインタ先の所有権がs2_heapに移ってるためコンパイルエラー
    // コピーしたら所有権が別のメモリに移るため解決策の一つ
    // コピー: let s2_heap = s_heap.clone();
    // println!("{}", s_heap);  // エラー
    println!("{}", s2_heap);  // 成功

    // スタックの場合，shallow copy(コピー)をするため所有権の競合がない
    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);

    /* 所有権と関数 */
    let str = String::from("Hello");  // strがヒープで確保
    takes_ownership(str);
    // ここからstrは無効

    let z = 5;
    makes_copy(z);
    // zはまだ有効

    let str = String::from("hello");
    let str = takes_and_gives_back(str);
    let (str, strlen) = cal_str_len(str);
    println!("{}: {}", strlen, str);

    /* 参照(readonly) */
    let hello_len = cal_str_size(&str);
    println!("{}: {}", hello_len, str);

    let mut hello = gives_ownership();
    push_world(&mut hello);
    println!("{}", hello);

    {
        let tmp_hello1 = &mut hello;
        tmp_hello1.push_str(" tmp1!");
        println!("tmp_hello1 has ownership {}", tmp_hello1);
    }

    let tmp_hello2 = &mut hello;
    tmp_hello2.push_str(" tmp2!");
    println!("tmp_hello2 has ownership {}", tmp_hello2);

    let mut new_s = dangle();
    new_s.push_str("new()");
    println!("{}", new_s);

    /* スライス */
    let hello_world = String::from("hello world");
    let end_first_word_index = first_word(&hello_world);
    println!("index: {}", end_first_word_index);

    let hello = &hello_world[0..5];
    let world = &hello_world[6..11];
    println!("{} {}", hello, world);

    let good_hello = good_first_word(&hello_world[..]);
    println!("{}", good_hello);

    let word_literal = "hello world";
    let literal_hello = good_first_word(word_literal);
    println!("{}", literal_hello);

    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];  // 参照で&[i32]型になる
}

fn takes_ownership(string: String) {  // ここでstrと同じメモリにアクセスしたいstringが現れるため，所有権がstringに移る
    println!("{}, world!", string);
}  // ここでstring(元= str)がスコープを抜け，dropが呼ばれる
// つまり，strで確保されていたメモリが解放される

fn makes_copy(integer: i32) {  // zからコピーした値をintegerに受ける
    println!("{}", integer);
}  // integerがスコープ外になるが，特別何もしない

fn gives_ownership() -> String {
    let hello_str = String::from("hello");

    hello_str  // 所有権は呼び出し元関数に移る
}

fn takes_and_gives_back(arg_str: String) -> String {
    // 引数として渡されたポインタの所有権は一度arg_strに移る
    // 引数のarg_strをそのまま返り値として返す
    // 呼び出し元に所有権が移る
    arg_str
}

fn cal_str_len(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

fn cal_str_size(s: &String) -> usize {
    // sは元の変数のメモリアドレスを受け取ってるだけ
    // s -> str -> "hello";
    // 所有権は受け取ってないのでスコープを外れても"hello"の解放はない
    s.len()
}

fn push_world(s: &mut String) {
    s.push_str(", world!")
}

// ヌルポ
// fn dangle() -> &String {
//     let str = String::new();
//     &str
// }
// 修正
fn dangle() -> String {
    let str = String::new();
    str
}

// 不便バージョン
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter()で各要素を返す
    // emurate()で(要素番号, 要素の参照)のタプルを返す
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// スライス利用後
// fn good_first_word(s: &String) -> &str {
fn good_first_word(s: &str) -> &str {  // さらに改善
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

