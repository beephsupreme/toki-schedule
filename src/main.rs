use scraper::Selector;
use std::{fs::File, io::Write};

const URL: &str = "https://www.toki.co.jp/purchasing/TLIHTML.files/sheet001.htm";

fn main() -> std::io::Result<()> {
    //
    let response = reqwest::blocking::get(URL).unwrap().text().unwrap();
    let html = scraper::Html::parse_document(&response);
    let selector = Selector::parse("td").unwrap();

    let mut elements: Vec<String> = Vec::new();

    for element in html.select(&selector) {
        let s = element.inner_html().to_string();
        let s = s.replace("\n", "");
        elements.push(s);
    }

    let mut file = File::create("foo.txt")?;

    for element in elements {
        file.write_all(element.as_bytes())?;
        file.write_all(b"\n")?;
    }
    return Ok(());
}
