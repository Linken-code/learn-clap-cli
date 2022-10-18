use dialoguer::{theme::ColorfulTheme, Confirm, Input};

//输入框
fn main() -> std::io::Result<()> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Tea or coffee?")
        .with_initial_text("Yes")
        .default("No".into())
        .interact_text()?;

    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') {
                Ok(())
            } else {
                Err("This is not a mail address")
            }
        })
        .interact()
        .unwrap();
    Ok(())
}
