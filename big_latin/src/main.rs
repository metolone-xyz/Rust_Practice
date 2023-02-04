/*
 * 文字列をビッグ・ラテンに変換するプログラム 
 * 各単語の最初の子音は、単語の終端へ移動し ay が追加 first →  irst-fay
 * 母音で始まる単語には hay が追加 apple →  apple-hay
 */
use std::io;
fn main() {
    let  mut user_input = get_input();
    let v: Vec<char> = user_input.chars().collect();
    match v.first() {
        Some(&'a') | Some(&'i') | Some(&'u') | Some(&'e') | Some(&'o') => {
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
