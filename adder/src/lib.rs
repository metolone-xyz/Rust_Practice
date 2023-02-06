/*引数に2を加えて結果を返すadd_two関数*/
pub fn add_two(a: i32) -> i32 {
    println!("受け取った引数は{}です",a);
    a + 8
} 

/* #[cfg(test)]はコンパイラに cargo build を走らせたときではなく
 * cargo testを走らせたときだけテストコードをコンパイルし走らせるように指示する*/
#[cfg(test)]
mod tests {
    /* use モジュール名::*
      とすれば指定したモジュール内で定義されている要素すべてを接頭語なしで参照できる*/
    use super::*;
//テスト関数であることを示す
     #[test]
    fn pass(){
        /*２つの引数を比べ、等しいか等しくないかを判断する*/
        assert_eq!(10,add_two(2));
    }
      #[test]
    fn pass1(){
        /*２つの引数を比べ、等しいか等しくないかを判断する*/
        assert_eq!(10,add_two(2));
    }
        
    }



