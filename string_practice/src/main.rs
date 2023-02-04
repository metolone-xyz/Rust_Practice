fn main() {
    //新規文字列を生成
    let mut s = "foo".to_string();

    //文字列を追加
    let s2 = "bar";
    //語尾に追加される
    s.push_str(s2);

    //ｓ２は所有権を奪われていないため使用できる
    println!("s:{}, s2:{}",s,s2);
}
