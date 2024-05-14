use std::io;
use std::path::Path;
use copy_dir;
fn main() -> io::Result<()> {
    let src_path = Path::new("./A");
    let dst_path = Path::new("./D");
    copy_dir::copy_dir_to(&src_path, &dst_path)?;
    println!("Directory copied successfully");
    Ok(())
}