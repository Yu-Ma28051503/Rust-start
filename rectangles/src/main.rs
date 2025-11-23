/* 構造体の利用例 */

#[derive(Debug)]
struct Rectangle {
    height: u32,
    weight: u32,
}

impl Rectangle {
    fn cal_area(&self) -> u32 {
        self.height * self.weight
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.weight > another.weight && self.height > another.height
    }
}

// 複数に分けることができる
// 有用な時が来るらしい
impl Rectangle {
    // 関連関数
    // 引数にselfをとらない
    fn squer(size: u32) -> Rectangle {
        Rectangle { weight: size, height: size }
    }
}

fn main() {
    println!("Hello, world!");
    let height = 35;
    let weight = 80;
    let rect1 = (height, weight);
    let rect2 = Rectangle { weight: 1024, height: 915 };
    let rect3 = Rectangle { weight: 512, height: 912 };
    let sqr = Rectangle::squer(10);  // 関連関数の呼び出し

    println!("area: {}", cal_area(height, weight));
    println!("area: {}", cal_tup_area(rect1));
    println!("area: {}", cal_struct_area(&rect2));
    println!("Structure Debug: {:#?}", rect2);
    println!("self method: {}", rect2.cal_area());
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));
    println!("squer area: {}", sqr.cal_area());
}

fn cal_area(h: u32, w: u32) -> u32 {
    h * w
}

fn cal_tup_area(dementions: (u32, u32)) -> u32 {
    dementions.0 * dementions.1
}

fn cal_struct_area(rect: &Rectangle) -> u32 {
    rect.height * rect.weight
}
