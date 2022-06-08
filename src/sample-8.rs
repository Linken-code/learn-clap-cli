/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 10:54:41
 * @LastEditTime: 2022-06-08 10:54:42
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Network port to use
    #[clap(parse(try_from_str))]
    port: usize,
}
///验证值 8080
fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}
