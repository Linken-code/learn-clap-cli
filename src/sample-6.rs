/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 10:35:13
 * @LastEditTime: 2022-06-08 10:35:29
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    //默认值
    #[clap(default_value_t = String::from("alice"))]
    name: String,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
