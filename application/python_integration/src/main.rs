use std::fs::File;
fn main() -> std::io::Result<()> {
    let mut f = File::create("foo.txt")?;
    println!("Hello");
    Ok(())
}