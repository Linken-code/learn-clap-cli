/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 10:51:19
 * @LastEditTime: 2022-06-08 10:52:11
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */

use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[clap(arg_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Fast,
    Slow,
}
// 模式匹配 fast/slow
fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }
}
