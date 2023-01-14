use  std::io;
fn scan() -> bool{
    let mut jud = String::new();
    io::stdin().read_line(&mut jud).expect("error");
   let jud: u32 = jud.trim().parse().expect("error");
   if jud == 1{
    true
   }else{
    false
   }
}
fn main() {
    let  count = 0;
    println!("数字を入力");
    let u = scan();
    if u == true {
        println!("レアドロップ!!");
    }else{
        println!("{}",count);
    }
}
