use std::io;
struct Rectangle{
    w:u32,
    h:u32,
}
impl Rectangle{
    fn area(&self) -> u32{
        self.w * self.h
    }

    fn can_hold(&self,other: &Rectangle) -> bool{
        if self.w > other.w && self.h > other.h{
            true
        }else{
            false
        }
    }
}


fn scan()->u32{
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("error");
    let num:u32 = num.trim().parse().expect("error");
    num
}
fn main() {
    println!("長方形の面積を求めます");
    println!("横幅↓");  let width = scan();
    println!("高さ↓"); let height = scan();
    let rec1 = Rectangle{w:width,h:height};
    let rec2 = Rectangle { w: 10, h: 20 };    
    let rec3 = Rectangle { w: 100, h:500 };
    println!("面積は{}です",rec1.area());
    println!("rec2はrec1にはまり込む {}",rec1.can_hold(&rec2));
    println!("rec3はrec1にはまり込む {}",rec1.can_hold(&rec3));
}
