
pub trait Summary {//トレイとを定義しないと使えない!!
    fn get_price(&self) -> i32;//&self メソッドが呼び出されている構造体インスタンス
}
pub struct Book{//本の構造体
    pub price: i32,
}

pub struct Game{
    pub price: i32,
    pub tax: i32,
}

 impl Summary for Book {
    fn get_price(&self) -> i32{
        self.price
    }
}

impl Summary for Game {
    fn get_price(&self) -> i32{
        &self.price + &self.tax
    }
}

fn main() {
    let book = Book{price: 5000};//インスタンスを生成
    let game = Game{price: 5000, tax: 100};

    println!("{}",book.get_price());
    println!("{}",game.get_price());
}
