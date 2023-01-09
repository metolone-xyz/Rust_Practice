use std::io;
fn main() {
   // let mut a = [0; 20];
    println!("何段目のフィボナッチ？");
    let mut step = String::new();
    io::stdin()
        .read_line(&mut step)
        .expect("error");
    let  step:i32 = step
        .trim()//空白を削除
        .parse()//指定した型に変換
        .expect("error");
    for i in 2..(step+1) {
        print!("{} ",fibonacci(i));
    }
    println!("");
    
}

fn fibonacci(n:i32) -> i32{
    return match n{
        0 => 0,
        1 => 1,
        _ => fibonacci(n-2)+fibonacci(n-1) 
    }
}

