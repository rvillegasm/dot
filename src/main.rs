use std::path::Path;

mod commands;
mod config;
mod files;

fn main() -> std::io::Result<()> {
    let from = Path::new("./test.txt");
    let to = Path::new("./test2.txt");

    files::rename(from, to)?;
    let link = files::symlink(to, from).unwrap(); // Do not panic!

    println!("{:?}", link);

    println!(
        "Created Symlink from {} to {}",
        from.display(),
        to.display()
    );

    Ok(()) // TODO: Manage error correctly later
}
