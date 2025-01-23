use std::fs::File;

mod line_read;
fn main() -> std::io::Result<()> {
    let f = File::open("hello.txt")?;

    let lines_vec = line_read::read_lines(f);

    println!("{:?}", lines_vec);
    Ok(())
}
