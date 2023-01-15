mod front_of_house;//ブロックではなくセミコロンを使うと同じ名前をしたファイルから読み込むという意味になる
pub use crate::front_of_house::hosting;


fn main() {
    hosting::add_to_waitlis();
}
