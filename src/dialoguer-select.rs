use console::Term;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

//选择项
fn main() -> std::io::Result<()> {
    let items = vec!["Item 1", "item 2"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which option do you prefer?")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything"),
    }

    let items = vec!["Item 1", "item 2", "item 3", "item 4"];
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .interact_on_opt(&Term::stderr())?;

    match selections {
        Some(positions) => println!("User selected options at indices {:?}", positions),
        None => println!("User exited using Esc or q"),
    }

    Ok(())
}
