//use std::io;
use rand::Rng;

fn main() {
    show_amida();
}

fn show_amida(){
    let answer = rand::thread_rng().gen_range(0..6);//正解の番号
    let mut posision: [i32; 5] = [0; 5];
    posision[answer] = 1;//当たりくじの場所に1を代入
    let mut _i = 0;
    let mut _j = 0;
    for _i in 0..6{
        print!("{}  ",_i);
    }
    println!(" ");
    _i = 0;
    for _j in 0..6{
        let horizontal_bar = rand::thread_rng().gen_range(0..5);
        if horizontal_bar == 1{
            println!("|--");
            posision.swap(_j, _j+1);//となりへ移動した分交換
        }else {
            print!("|  ");
        }
    } 
    println!(" ");
}
