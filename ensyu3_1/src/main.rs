use std::io;
fn main() {
    println!("２つの整数を入力してください");
    println!("整数A >"); 
    let a = scan();
    println!("整数B >"); 
    let b = scan();
    if a%b == 0{
        println!("{}は{}の約数です",b,a);
    }else{
        println!("{}は{}の約数ではありません",b,a);
    }
}

fn scan() ->u64{
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("error");
    let num:u64 = num.trim().parse().expect("error");
    num
}
