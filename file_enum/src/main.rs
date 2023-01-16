mod greeting;
use crate::greeting::g_module::Time;
fn aisatsu(time: Time){
    match time {
       Time::Morning => {println!("Good Morning")}
       Time::Afternoon => {println!("Good Afternoon")}
       Time::Evening => {println!("Good Evening")}
       Time::Night => {println!("Good Night")}
    }
}
fn main() {
let when = Time::Morning;
aisatsu(when);
}
