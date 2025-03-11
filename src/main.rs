use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;

    for i in 1..11 {
        file.write_all(i.to_string().as_bytes())?;
        file.write_all(b"\n")?;
    }
    return Ok(());
}
