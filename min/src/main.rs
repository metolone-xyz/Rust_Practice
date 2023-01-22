use std::io;
fn main() {
    println!("３つの数字の最小値を求めます");
    let n1 = get_input();
    let n2 = get_input();
    let n3 = get_input();
    let mut max = &n1;
    if &n2 < max {
        max = &n2;
    }
    if &n3 < max{
        max = &n3;
    }
    println!("最小の値は{}です", max);
}

fn get_input() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("error");
    let num: i32 = num.trim().parse().expect("error");
    num
}
