use std::io;
fn main() {
    println!("入力された整数以下である正の２のべき乗の数を順に表示します");
    let user_input = get_input();
    
    let mut rui = 1;
    while 2_u32.pow(rui) <= user_input{
        //_u32は符号なし32bit整数ということをあらわす。2が曖昧な数値型だとメソッドが使えない
        print!("{} ",2_u32.pow(rui));
        rui +=1; 
    }
    println!();
}

fn get_input() -> u32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: u32 = input.trim().parse().expect("error");
    input
}
