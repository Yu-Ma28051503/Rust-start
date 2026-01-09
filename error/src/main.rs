use std::io;
use std::fs::File;
use std::io::ErrorKind;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value 100 {
            panic!("Guess value must be between 1 and 100, get {}", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("world.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn ReadUsernameFromFile() -> Result<String, io::Error> {
    let mut f = File::open("word.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // 省略可能
    // let mut s = String::new();
    // File::open("word.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}

fn main() {
    println!("Hello, world!");
    //panic!("crash and burn!");  // クラッシュして炎上

    let v = vec![1, 2, 3];
    //v[99];  // 範囲外の要素へアクセスしpanic!

    // File::openの返り値がわからないときは，適当な型でコンパイルするとコンパイラが教えてくれる
    // = note: expected type `u32`
    //          found enum `Result<File, std::io::Error>`
    // let f: u32 = File::open("hello.txt");
    
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("ERROR: {:?}", error)
        },
    };

    /* エラー時にパニックするショートカット:unwrapとexpect */
    // unwrap()はOkならば中身を返し，Errならばpainic!を呼び出す
    let fp = File::open("hi.txt").unwrap();
    let ft = File::open("hit.txt").expect("Failed to open hit.txt");  // panic!のメッセージを入れれる

    /* エラーの委譲 */
    // fn read_username_from_file() -> Result<String, io::Error>

    /* エラー委譲のショートカット?演算子 */
    // fn ReadUsernameFromFile() -> Result<String, io::Error>
}
