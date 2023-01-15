mod user_data;
use crate::user_data::user_data::Data;
fn main() {
    let my_data = Data{user_name:String::from("Yamada Taro"),user_age:18,};
println!("{},{}",my_data.user_name,my_data.user_age);
}
