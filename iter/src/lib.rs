#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        //２つのイテレータを生成 zipメソッドで最初のCounterイテレータと
        //もう一つをペアにする
        //skipはイテレータから最初の要素をスキップする
        .zip(Counter::new().skip(1))
        //zipされたペアをかけ合わせる
        .map(|(a, b)| a * b)
        //mapで変換された値をフィルタリングする
        .filter(|x| x % 3 == 0)
        //フィルタリングされた値の総和を計算
        .sum();
}
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//イテレータトレイトを実装
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
