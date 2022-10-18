use dialoguer::{theme::ColorfulTheme, Confirm, Password};

//输入框
fn main() -> std::io::Result<()> {
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("New Password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?;
    println!("Length of the password is: {}", password.len());
    Ok(())
}
