use std::io;
fn main() {
    println!("2つの数字を入力してください");
    let a = scan();
    let b = scan();
    println!("{}と{}の積は{}です",a,b,a*b);
}

fn scan() -> i32{
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("エラー");
    let x:i32 = x.trim().parse().expect("エラー");
    x
    
}
