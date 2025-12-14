use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    /* ベクター */
    let v: Vec<i32> = Vec::new();
    let ve = vec![1, 2, 3, 4, 5];
    let mut vmut = Vec::new();

    vmut.push(5);
    vmut.push(6);

    {
        let vm = vec![1, 2, 3, 4];
    }

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let mut vect = vec![100, 32, 57];
    for i in &mut vect {
        println!("{}", i);
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /* String */
    // let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // 以下全て有効
    // どのゲンンゴの文字列でもUTF-8に変換される
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    let mut foo = String::from("foo");
    let bar = String::from("bar");
    foo.push_str(&bar);  // 複数文字を連結できる
    println!("{}", foo);

    let mut lol = String::from("lo");
    lol.push('l');  // 1文字追加のみ

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world;  // helloは所有権が移り利用不可になる

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // formatマクロで複雑な文字列連結
    let ttt = format!("{}-{}-{}", tic, tac, toe);

    /* HashMap */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let score: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let blue_score = scores.get(&String::from("Blue"));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 30);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_map);
}
