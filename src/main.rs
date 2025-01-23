use std::fs::File;

mod line_read;
fn main() -> std::io::Result<()> {
    let f = File::open("hello.txt")?;

    let mut lines_vec = line_read::read_lines(f);

    println!("{:?}", lines_vec);
    Ok(())
}
