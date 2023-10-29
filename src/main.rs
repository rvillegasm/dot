use std::path::Path;

use crate::symlink::symlink;

mod symlink;

fn main() {
    let from = Path::new("./test.txt");
    let to = Path::new("./test2.txt");

    let link = symlink(from, to).unwrap(); // Do not panic!

    println!("{:?}", link);

    println!(
        "Created Symlink from {} to {}",
        from.display(),
        to.display()
    );
}
