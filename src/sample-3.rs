/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 10:02:42
 * @LastEditTime: 2022-06-08 10:02:42
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)] //派生宏控制是否有前缀
    name: Option<String>,
}
// 有前缀-n bobo
//无前缀 bobo
fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name.as_deref());
}
