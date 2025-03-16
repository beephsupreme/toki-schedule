//

mod schedule;

fn main() -> std::io::Result<()> {
    schedule::schedule().unwrap();
    return Ok(());
}
//
