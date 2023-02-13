use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    //子供に親の存在を気づかせるためのparentフィールド
    //子ノードがドロップ→ 親はドロップされない
    //親がドロップ→ 子もドロップ させるため弱い参照
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    //子供なしのNodeインスタンス
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf parent befor instance branh {:?}",
        //upgrade weakの情報をOption<Rc<T>>で返す
        leaf.parent.borrow().upgrade()
    );
    //leafを子要素の1つとして持つインスタンス
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    //leafのparentを示すポインタを変異的に借用
    //downgrade はWeak<T>のスマートポインタを得る
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
        "leaf parent after instance branh {:?}",
        leaf.parent.borrow().upgrade()
    );
}
