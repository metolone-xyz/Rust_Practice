use std::io;

fn main() {
    println!("入力された数字までの偶数を求めます");
    let user_input = get_input();
    for i in 2..=user_input{
        if i%2 == 0{
            print!("{} ",i);
        }
    }
    println!();
}

fn get_input() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: i32 = input.trim().parse().expect("error");
    input
}
