use std::env;
use std::error::Error;
//コマンドライン引数の値を読み取れるようにする
use std::fs::File; //ファイルを扱えるようにする
use std::io::prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            serch_case_insensitive(query, contents)
        );
    }
}

//返す参照がどちらを参照すればいいかわからないからライフタイム注釈が必要
//今回serch関数から返される値はcontents引数に渡されるデータと同期間生きる
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn serch_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); //小文字化
    let mut results = Vec::new();
    for line in contents.lines() {
        //&がない場合、to_lowercaseは新しいstring型を生成するためメモリを確保しなければいけない
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

//構造体は要素も公開しなければならない
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, //大文字、小文字を区別しないで調べるか区別して調べるかをコマンドラインから選択できるようにする
}

/*コマンドライン引数を受け取るConfig構造体に紐づくメソッド
 * 引数の値を変数に保存
 * args[0] ... プログラム名
 * args[1] ... 検索する文字列
 * args[2] ... ファイル名
##########################################################
*is_err ・・・CASE_INSENSITIVEに環境変数がセットされていればis_errはfalseを返し,大文字と小文字を区別しないで検索
 */
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        //env::argsの戻り値の1番目はプログラム名
        args.next();

        //queryフィールドに置きたい値を得る
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        //filenameフィールドに置きたい値を得る
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let input_case = env::var("CaseInput").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive: input_case,
        })
    }
}

/*Okのときはユニット型(フィールドを持たない構造体や列挙型のバリアント)を返し、エラーのときはトレイトオブジェクトを使用*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /*File::open関数を呼んでfilename変数の値に渡し、可変なハンドルを得る*/
    let mut f = File::open(config.filename)?;

    /*contents という名の変数を生成*/
    let mut contents = String::new();
    /*ファイルハンドルに対してread_to_stringを呼び出し、引数として contentsへの可変参照を渡す
     * ?演算子は呼び出し元が処理できるように現在の関数からエラー値を返す
     */
    f.read_to_string(&mut contents)?;

    //case_sensitiveフィールドを確認してserchかserch_case_insensitiveか分岐
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        serch_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(()) //Okのときはユニット型を返す
}
