extern crate add_one;
extern crate add_two;

fn main() {
    let num = 10;
    println!("{}, {}", num, add_one::add_pne(num));
    println!("{}, {}", num, add_two::add_two(num));
}
