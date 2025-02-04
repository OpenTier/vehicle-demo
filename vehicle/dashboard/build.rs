use std::io::Result;

fn main() -> Result<()> {
    slint_build::compile("ui/main_window.slint").unwrap();
    Ok(())
}
