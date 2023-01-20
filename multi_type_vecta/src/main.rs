#[derive(Debug)]
enum MultiType{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<MultiType> = Vec::new();
    v.push(MultiType::Int(4));
    v.push(MultiType::Float(1.1));
    v.push(MultiType::Text(String::from("blue")));
       
    for i in &v {
        println!("{:?}, ", i);
    }
}
