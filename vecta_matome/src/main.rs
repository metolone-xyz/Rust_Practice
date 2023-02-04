/* 整数のリストが与えられ、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返すプログラム*/
use std::collections::HashMap;
fn main() {
    let mut v = vec![4,3,1,7,5,6,8,4,4,4,4,4];
   
    //平均値
    println!("mean: {}", average(&v));
 
    //ソートされたときに真ん中に来る値
    println!("median: {}", median(&mut v));

    //最も頻繁に出現する値
    let most = mode(&v);
    println!("最も出現する値は {} で {}回出現しました",most.0, most.1);
}
fn mode(v: &Vec<i32>) -> (&i32, i32){
    let mut modes = HashMap::new();

    for value in v {
        let count = modes.entry(value).or_insert(0);
        *count += 1;
    }

    let mut tap: (&i32, i32) = (&0,0);

    for (key,count) in modes{
        if count > tap.1 {
            tap.1 = count;
            tap.0 = key;
        }
    }
    tap
}
/*平均値を求めるプログラム*/
fn average(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v{
        sum+=value;
    }
    sum/v.len() as i32
}

fn median(v: &mut Vec<i32>) -> i32{
   insert_sort(v) ;
   v[v.len() / 2]
}

//挿入ソートを行う関数
fn insert_sort(v: &mut Vec<i32>){
    for i in 1..v.len(){
       let mut j = i;
       while j > 0 && v[j -1] > v[j] {
           v.swap(j-1, j);
           j-=1;
       }
    }
    
    for i in v {
        print!("{}",i);
    }
   println!() ;
}

