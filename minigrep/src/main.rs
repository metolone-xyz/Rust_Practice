/*
 * extern crate はプログラムから外部のRust crate（ライブラリ）を参照する記述
 */
extern crate minigrep;
use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;
fn main() {
    /*
     * コマンドラインから引数を受け取る
     * std::envモジュールを導入したためargsが使用できる
     * argsはコマンドライン引数を受け取る
     * collect ・・・ args関数の戻り値をベクタ型で処理する
     */
    let args: Vec<String> = env::args().collect();

    /*
     * unwrap_or_else ・・・panic!ではない独自のエラー処理を定義できる
     *|err| クロージャ 今回は 引数の文字列が足りませんというErrの中身がここに:渡される
     */
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
