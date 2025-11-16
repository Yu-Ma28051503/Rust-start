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
