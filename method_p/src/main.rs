/*
 *メソッド記法
 */
struct Rectangle {
    width: u32,
    height: u32,
}
//ジェネリックな型を扱う構造体
struct Point<T> {
    x: T,
    y: T,
}
/*
 * 構造体の文脈内で関数を定義するには impl(implementation: 実装)で始める。
 * コンパイラはメソッドが構造体の文脈内に存在することを把握するため &self を使用する(rectangle: &Rectangle)
 * メソッドは self の所有権を奪ったり、不変または可変で値を借用もできる*/
impl  Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    
}

impl<T> Point<T>{
    fn sum(&self) -> &T {
        &self.x
    }
}
fn main() {
    let rect1 = Rectangle {width: 40, height: 50};
    
    let rect2 = Point {x: 40, y: 50};

    println!("{}",rect1.area());
    println!("{}",rect2.sum());
}
