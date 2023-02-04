//標準ライブラリのコレクションからuseする必要あり
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 80);

    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 40];

    //zipメソッドを使いタプルのベクタを作る→　collectメソッドでベクタをタプルのハッシュマップに変換
    let score: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //ハッシュマップの値を取り出す
    for (key, value) in &score {
        println!("{}, {}",key, value);
    }
}
