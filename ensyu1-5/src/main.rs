use std::io;
fn main() {
    println!("数字を入力");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("読み込みに失敗");
    let  n:i32 = n.trim()
        .parse()
        .expect("エラー");
    println!("{}に12加えると{}です",n,n+12);
}
