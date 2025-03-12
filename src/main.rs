//
use crate::URL;

use scraper::Selector;
use std::{fs::File, io::Write};

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

    let mut date_file = File::create("dates.txt")?;
    for i in 0..number_of_dates {
        date_file.write_all(elements[i].as_bytes())?;
        date_file.write_all(b"\n")?;
    }

    elements.drain(0..(2 * number_of_dates + 5));

    for element in elements.iter_mut() {
        if element == "�@" {
            *element = "0".to_string();
        }
    }

    let mut records: Vec<String> = Vec::new();

    let cols = number_of_dates + 5;
    let rows = elements.len() / cols;

    for r in 0..(rows - 1) {
        for c in 0..cols {
            if c == 0 || c > 4 {
                records.push(elements[cols * r + c].clone());
            }
        }
    }

    let mut records_file = File::create("records.txt")?;
    for record in records {
        records_file.write_all(record.as_bytes())?;
        records_file.write_all(b"\n")?;
    }

    return Ok(());
}
//
