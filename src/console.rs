use console::Term;
use console::{style, Emoji, Style};
use std::error::Error;
use std::thread;
use std::time::Duration;

//颜色输出
fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.write_line("Hello World!")?;
    thread::sleep(Duration::from_millis(2000));
    term.clear_line()?;
    println!("This is {} neat", style("quite").cyan());
    let cyan = Style::new().green();
    println!("This is {} neat", cyan.apply_to("quite"));
    println!("[3/4] {}Downloading ...", Emoji("🚚 ", "--"));
    println!("[4/4] {} Done!", Emoji("✨", ":-)"));

    Ok(())
}
