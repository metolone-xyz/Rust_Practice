/*
 * 文字列をビッグ・ラテンに変換するプログラム 
 * 各単語の最初の子音は、単語の終端へ移動し ay が追加 first →  irst-fay
 * 母音で始まる単語には hay が追加 apple →  apple-hay
 */
use std::io;
fn main() {
    // 文字列を入力
    let  mut user_input = get_input();

    //入力された文字列をchar型ベクタへ変換
    let v: Vec<char> = user_input.chars().collect();

    /*
     * 先頭が母音のときと子音のときで処理を分ける
     * first()メソッドはベクタの最初の要素への参照を取得する。
     * first()メソッドはOption<&char>を返すためベクタが空のときも処理できる
     * &v[0]はインデックスが範囲外になりプログラムがパニックになる 
     * ベクタ v が１つでも要素を持っている場合が確実なときは &v[0]は使用可能
     */
    match v.first() {
        Some(&'a') | Some(&'i') | Some(&'u') | Some(&'e') | Some(&'o') => {

            /*
             * String型は最後に改行文字を含むためそのままだと - の前で改行される
             * そのためreplaceで改行文字を削除
             */
            print!("{}-hay",user_input.replace("\n", ""));
            println!();
        },
        Some(_) => {  
            user_input.remove(0);
            print!("{}-{}ay", user_input.replace("\n", ""), v[0]);
            println!();
        },
        None => println!("error"),
    }
}


fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input
}
