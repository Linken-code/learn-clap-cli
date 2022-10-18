use dialoguer::Editor;
//文件编辑
fn main() -> std::io::Result<()> {
    if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap() {
        println!("Your message:");
        println!("{}", rv);
    } else {
        println!("Abort!");
    }
    Ok(())
}
