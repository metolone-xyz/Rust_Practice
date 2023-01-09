fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..5{
    v.push(i+1);
    }
    for i in &v {//参照を得てそれらを表示
        println!("{}",i);
    }

}
