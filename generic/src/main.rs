/*ジェネリックな型をもつタプルを作る関数*/
fn make_touple<T, U>(t: T, u: U) -> (T, U) {
    (t, u)
}

fn main() {
    let number_list = vec![2, 5, 1, 6, 9];

    println!(
        "The largest number of this vector is {}",
        largest(&number_list)
    );

    /*整数型のタプルを作成*/
    let tou_i32 = make_touple(4, 3);
    println!("整数型: {}, {}", tou_i32.0, tou_i32.1);

    /*浮動小数点型のタプルを作成*/
    let tou_f64 = make_touple(2.5, 1.8);
    println!("浮動小数点型: {}, {}", tou_f64.0, tou_f64.1);

    let mix = make_touple(3, 2.4);
    println!("ミックスされたタプル: {}, {}", mix.0, mix.1);
}

/*最大値を求めるプログラム
 * 引数にはスライス型を借用する
 *スライス型は任意の長さの一連の要素を表すことができるため配列やベクタ型を参照することもできる*/
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    /*.iter()はイテレータを作成
     * イテレータとは要素をひとつひとつ巡っていく操作を抽象化したもの
     */
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*ジェネリックなlargest関数
 * Tは比較不能な型が来たときのためにPartialOrdで比較可能にする
 * Copyトレイトを実装していない型が来たときのためにCopyで値をムーブできるようにする*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    /*.iter()はイテレータを作成
     * イテレータとは要素をひとつひとつ巡っていく操作を抽象化したもの
     */
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
