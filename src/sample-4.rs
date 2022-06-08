/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 10:09:29
 * @LastEditTime: 2022-06-08 10:09:30
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    //派生宏控制是否布尔
    // 有前缀-v 为true
    //无前缀 为false
    #[clap(short, long)]
    verbose: bool,
    //派生宏统计引用次数
    #[clap(short, long, parse(from_occurrences))]
    num: usize,
}
//-v -nnn
fn main() {
    let cli = Cli::parse();
    println!("verbose: {:?}", cli.num);
    println!("verbose: {:?}", cli.verbose);
}
