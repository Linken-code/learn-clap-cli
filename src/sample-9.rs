/*
 * @Author: yhlz-Linken
 * @Date: 2022-06-08 14:21:33
 * @LastEditTime: 2022-06-08 14:21:34
 * @LastEditors: yhlz-Linken
 * yhlz.com
 */
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("vers")
                .required(true)
                .args(&["set-ver", "major", "minor", "patch"]),
        ))]
struct Cli {
    /// set version manually
    #[clap(long, value_name = "VER")]
    set_ver: Option<String>,

    /// auto inc major
    #[clap(long)]
    major: bool,

    /// auto inc minor
    #[clap(long)]
    minor: bool,

    /// auto inc patch
    #[clap(long)]
    patch: bool,

    /// some regular input
    #[clap(group = "input")]
    input_file: Option<String>,

    /// some special input argument
    #[clap(long, group = "input")]
    spec_in: Option<String>,

    #[clap(short, requires = "input")]
    config: Option<String>,
}
//参数分组不能同时使用
//--major/--minor/--patch
//分别对应版本1.1.1
//--set-ver 1.2.3设置版本为1.2.3
//-c输入必须参数 <INPUT_FILE|--spec-in <SPEC_IN>>
//--major -c config.toml --spec-in input.txt
//--major -c config.toml tst
fn main() {
    let cli = Cli::parse();

    // Let's assume the old version 1.2.3
    let mut major = 1;
    let mut minor = 1;
    let mut patch = 1;

    // See if --set-ver was used to set the version manually
    let version = if let Some(ver) = cli.set_ver.as_deref() {
        ver.to_string()
    } else {
        // Increment the one requested (in a real program, we'd reset the lower numbers)
        let (maj, min, pat) = (cli.major, cli.minor, cli.patch);
        match (maj, min, pat) {
            (true, _, _) => major += 1,
            (_, true, _) => minor += 1,
            (_, _, true) => patch += 1,
            _ => unreachable!(),
        };
        format!("{}.{}.{}", major, minor, patch)
    };

    println!("Version: {}", version);

    // Check for usage of -c
    if let Some(config) = cli.config.as_deref() {
        let input = cli
            .input_file
            .as_deref()
            .unwrap_or_else(|| cli.spec_in.as_deref().unwrap());
        println!("Doing work using input {} and config {}", input, config);
    }
}
