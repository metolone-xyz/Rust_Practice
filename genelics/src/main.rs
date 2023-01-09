fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list{
        if item > largest{//Largest関数があらゆる型のスライスに対して動くようにするためにはPartialOrdを指定する必要あり
                          //i32やcharはCopyトレイとを実装済み
                          //しかしlargest関数をジェネリックにするとlist引数がCopyを実装しない方を含む可能性があるためトレイト境界にはPartialOrdとCopyを実装する限りコンパイルできるように指定
            largest = item;
        }
    }
    largest
}

fn main(){
    let number_list = vec![1,41,25,89,45];
    println!("{}", largest(&number_list));
}
