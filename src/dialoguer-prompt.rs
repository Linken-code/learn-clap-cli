use dialoguer::{theme::ColorfulTheme, Confirm};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .default(true)
        .with_prompt("Do you wish to continue?")
        .interact()?;

    if proceed {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    Ok(())
}
