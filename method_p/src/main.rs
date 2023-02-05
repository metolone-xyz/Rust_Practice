use std::ops::Mul;

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

/*
 * ジェネリックな型は演算子*をサポートしていない
 * where T: Mul<Output = T> は 型T が 演算子をサポートするように要求している（この場合はstd::ops::Mul）
 * Output = T は演算子の結果の方を指定するもの これにより * で生成される値の型が Tであることが保証される
 * whereは関数やメソッドなどにシグネチャを付与するもので、ジェネリックな型やトレイとなどの条件を指定するために使用
 * Copyトレイとを実装していない型を含む可能性もあるためトレイと境界にCopyを追加*/
impl<T> Point<T> where T: Mul<T, Output = T> + Copy{
    fn sum(&self) -> T {
        self.x * self.y
    }
}
fn main() {
    let rect1 = Rectangle {width: 40, height: 50};
    
    let rect2 = Point {x: 40, y: 50};

    println!("{}",rect1.area());
    println!("{}",rect2.sum());
}
