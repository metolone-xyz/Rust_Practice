use std::io;
fn main() {
    println!("素数かどうかを確かめます");
    let user_input = get_input();
    match user_input {
        0..=1 => println!("false"),
            _ => {
                for _i in 2..user_input{
                    if user_input%_i == 0{ println!("素数ではありません"); std::process::exit(1);}
                }
                println!("素数です");
            }
    }
}

fn get_input() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: i32 = input.trim().parse().expect("error");
    input
}
