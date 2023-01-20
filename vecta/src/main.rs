
fn main() {
    //ベクタを生成
    //何も要素がないベクタを生成するときはコンパイラに保持したいデータの種類を教えるために型注釈をつける
   let mut v: Vec<i32> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(3);
    v.push(4);

    let first = &v[0];
    println!("{}",first);
    let get = v.get(0);
    println!("{:?}",get);

    match v.get(0) {
       Some(i) => println!("1番目の要素は{}です",i),
        None => println!("要素なし"),
    }

    //ベクタ内の値を順に処理
    for i in &v {
        print!("{}  ", i);
    }
    println!("");
    //ベクタは同じ型の値しか保持できないが、enumの列挙子ごとに異なる値を保持するようにすると複数の型を保持できるようになる
    //enumの列挙子はすべて同じ型、つまりenumの型として考えられるから
    let row = vec![
        SpreadsheetCell::Int(5),
                                        SpreadsheetCell::Float(1.1),
                                        SpreadsheetCell::Text(String::from("blue")),
    ];
    for i in &row{
        print!("{:?}  ",i);
    }
}
#[derive(Debug)]
//enumを使って複数の型を保持する
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
