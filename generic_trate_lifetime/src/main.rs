struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Points<T, U> {
    x: T,
    y: U,
}

impl<T, U> Points<T, U> {
    fn mixup<V, W>(self, other: Points<V, W>) -> Points<T, W> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}

fn fn_largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn fn_largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn fn_largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn longest<'a>(x: &'a str, y: &'a<> str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn fiset_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    println!("Hello, world!");

    /* ジェネリック */
    /* 関数を抽出することで重複を取り除く */
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {:?}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {:?}", largest);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = fn_largest_i32(&number_list);
    println!("The largest number is {}", result);

    /* ゲネリックなデータ型 */
    /* 関数定義では */
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = fn_largest_char(&char_list);
    println!("The largest char is {}", result);

    /* 構造体定義では */
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }
    let intenger = Point{ x : 5, y: 4 };
    let float = Point{ x : 1.0, y : 2.4 };
    // let epoint = Point{ x : 10, y : 1.0 };  // 型は同じでないといけない

    // 異なる型にする場合
    // struct Points<T, U> {
    //    x: T,
    //    y: U,
    //}
    let both_int = Points{ x : 6, y : 12 };
    let both_float = Points{ x : 1.1, y : 4.4 };
    let i_and_f = Points{ x : 4, y : 100.5};

    /* enum定義では */
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    /* メソッド定義では */
    let p = Point{ x : 5, y : 10 };
    println!("p.x = {}", p.x());

    let p1 = Points{ x : 5, y : 10.4 };
    let p2 = Points{ x : "HELLO", y : 'w' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    /* ジェネリクスを利用したコードのパフォーマンス */
    let intenger = Some(5);
    let intenger = Option_i32::Some(5);  // このようにコンパイラが置き換える
    let float = Some(5.0);
    let float = Option_f64::Some(5.0);  // コンパイラがこのように置き換える

    /* トレイト: 共通の振る舞いを定義する */
    let tweet = Tweet {
        username: String::from("horse_ebboks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Uceburgh"),
        content: String::from("The Pittsburgh Penguins once agin are the best hockey team in th NHL."),
    };

    println!("New article available! {}", article.summarize());

    /* ライフタイムで三勝を検証する */
    /* ライフタイムでダングリング参照を回避する */
    {
        let r;
        let x = 5;
        r = &x;

        println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), stirng2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not fine a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
