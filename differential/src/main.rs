//微分をするプログラム
fn main() {
    println!("{}",diff(1.0));
}

//関数 f(x)=2x
fn f(x: f64) -> f64{
    2.0 * x
}
//微分
   fn diff(a: f64) -> f64{
       (f(a+0.001) - f(a))/0.001
   }
   
