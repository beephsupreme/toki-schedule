use scraper::Selector;
use std::{fs::File, io::Write};

const URL: &str = "https://www.toki.co.jp/purchasing/TLIHTML.files/sheet001.htm";

fn main() -> std::io::Result<()> {
    // let mut file = File::create("foo.txt")?;
    //
    // for i in 1..11 {
    //     file.write_all(i.to_string().as_bytes())?;
    //     file.write_all(b"\n")?;
    // }
    let response = reqwest::blocking::get(URL).unwrap().text().unwrap();
    let html = scraper::Html::parse_document(&response);
    let selector = Selector::parse("td").unwrap();
    let mut elements: Vec<String> = Vec::new();
    for element in html.select(&selector) {
        println!("{}", element.inner_html());
        // elements.push(element.inner_html());
    }

    return Ok(());
}
