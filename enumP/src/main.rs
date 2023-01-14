fn main() {
    let  se:Option<String> = None;
   // se = Some(String::from("hello")); // コンパイラは上の文を読まずにコンパイルするためwaringが出る
    println!("{:?}",se);
}
