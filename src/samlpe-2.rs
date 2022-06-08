/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 09:35:54
 * @LastEditTime: 2022-06-08 10:09:31
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "MyApp")]
#[clap(author = "Kevin K. <kbknapp@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Does awesome things", long_about = None)]
struct Cli {
    #[clap(long)]
    two: String,
    #[clap(long)]
    one: String,
}
//--two as --one dv
fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}
