use std::io;
fn main() {
    println!("文字を2つ入力してください");
let a = scan();
let b = scan();
println!("{}は{}の{:.2}%です",a,b,(a/b)*100.0);
}

fn scan()->f64{//文字入力を受け取る関数
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("エラー");
    let num:f64 = num.trim().parse().expect("error");
    num
}
