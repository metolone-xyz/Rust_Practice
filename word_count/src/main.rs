//テキスト内に何回同じ単語が出てきたかを数える

use std::collections::HashMap;
fn main() {
    let text = "hello world wonderful world";

    //ハッシュマップを作成
    let mut map = HashMap::new();

    //空白で文字列を分割しハッシュマップを作成する
    for word in text.split_whitespace() {
        //初めて出てきた単語なら新たにハッシュマップを作成
        let count = map.entry(word).or_insert(0);

        //* はデリファレンス参照変数から実際の値を取得するための記法
        *count += 1;
    }
   
   let mut max_value = 0;
   let mut max_word = "a";
    for (key, value) in map{
        if value > max_value {
            max_value = value;
           max_word = key;
        }
    }


    println!("文字列 {} のなかで 最も多い単語は {} で {} 個含まれています",text, max_word, max_value);
}
