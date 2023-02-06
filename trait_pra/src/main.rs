fn main() {
    let news = NewsArticle {
        headline: String::from("Rust is the most favorite programming language"),
        location: String::from("Japan"),
        author: String::from("Kensi TAKADA"),
    };

    let my_name = Name {
        first_name: String::from("Kensi"),
        last_name: String::from("TAKADA"),
    };

    println!("{}", news.summarize());
    println!("{}", my_name.summarize());
}

/*トレイトを定義
 * トレイト名: Summary
 * 波カッコ内にはメソッドシグニチャ（型の振る舞いを定義）を定義*/
pub trait Summary {
    fn summarize(&self) -> String {
        //デフォルト実装
        String::from("default")
    }
}

pub struct Name {
    pub first_name: String,
    pub last_name: String,
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}
/*implのあとに実装したいトレイとの名前を置く*/
impl Summary for NewsArticle {
    /*独自の処理を実装*/
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Name {
    /* コメントを外すとオーバーライド（上書き）され独自の処理が実装sれる
    fn summarize(&self) -> String {
        format!("My name is {} {}", self.first_name, self.last_name)
    } */
}

/*引数としてのトレイト*/
pub fn notify(news: &impl Summary) {
    println!("Breaking news! {}", news.summarize());
}
