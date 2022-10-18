use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

//模糊搜索
fn main() -> std::io::Result<()> {
    let items = vec!["Item 1", "item 2", "item 3"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which option do you prefer?")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything"),
    }

    Ok(())
}
