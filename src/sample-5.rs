/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 10:28:30
 * @LastEditTime: 2022-06-08 10:28:45
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
}
///子命令
//add linken
fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
    }
}
