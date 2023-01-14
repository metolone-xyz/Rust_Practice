fn first_word(s: &str) -> &str{//文字列を受け取ってその文字列中の最初の単語を返す
    let bytes = s.as_bytes();//Stringの各値を見て空白かどうか確かめるためにバイト配列に変換

    for (i, &item)in bytes.iter().enumerate(){//enumerate()はイテレータが返す値と現在のインデックスのペアを作成して返す。インデックスはusize型
        if item == b' '{//もし空白が見つかったら
            return &s[..i];//文字列の最初からiのひとつ手前までスライス
        }
    }
    &s[..]//もし空白が見つからなければ文字列全体を返す
}
fn main() {
    let s = "hello world";
    let f = first_word(&s);//空白文字のインデックス
    //s.clear(); ← fで不変として借用されているため可変で借用できない
    println!("{}",f);//sの最初からf(空白文字の１つ手前)まで参照
}
