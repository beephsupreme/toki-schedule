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

    let mut toki_code: usize = 0;
    let mut first_date: usize = 0;

    for (i, s) in elements.iter().enumerate() {
        if s == "TOKISTAR CODE" {
            toki_code = i;
        }
    }
    for i in (0..toki_code).rev() {
        if elements[i].is_empty() {
            first_date = i + 1;
            break;
        }
    }

    let number_of_dates = toki_code - first_date;

    elements.drain(0..first_date);

    let mut file = File::create("dates.txt")?;
    for i in 0..number_of_dates {
        file.write_all(elements[i].as_bytes())?;
        file.write_all(b"\n")?;
    }

    return Ok(());
}
