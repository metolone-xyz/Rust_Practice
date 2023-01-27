//グローバルな定数を宣言
const N:i32 = 15;
fn era(a:&mut [i32]){
    a[0] = 0; 
    a[1] = 0;
    for i in 2..N{
        let mut j = 2;
        while i*j<N {
            a[(i as usize)*(j as usize)] = 0;
            j += 1;
        }

    }
}

fn show_array(arr: &mut [i32]){
    for i in arr{
        print!("{} ",i);
    }
    println!();
}

fn show_erat(arr: &mut [i32]){
    for i in arr{
        //if i != 0{ iは参照のためアドレスをもっている よって比較できない
        //*i を使うと参照変数が示す先の値を取得できる
        //参照変数から実際の値を取得することをデリファレンスという
        if *i !=0{
            print!("{} ",i);
        }
    }
    println!();
}
fn main() {
    println!("エラトステネスのふるい");
    let mut arr: [i32;15] = [0;15];
    for i in 0..N {
        arr[i as usize] = i;
    }
    show_array(&mut arr);
    era(&mut arr);
    show_erat(&mut arr);
}
