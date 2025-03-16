//
use regex::Regex;
use scraper::Selector;
use std::collections::HashMap;
use std::{fs::File, io::Write};
pub(crate) use toki_schedule::URL;

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

    // find & count shipping dates
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

    // Write dates to file
    let mut date_file = File::create("dates.txt")?;
    for i in 0..number_of_dates {
        date_file.write_all(elements[i].as_bytes())?;
        date_file.write_all(b"\n")?;
    }

    // Delete rows up to first line of part numbers
    elements.drain(0..(2 * number_of_dates + 5));

    // Remove Japanese characters
    for element in elements.iter_mut() {
        if element == "�@" {
            *element = "0".to_string();
        }
    }

    // Truncate on non-alpha-numeric (except dashes)
    for element in elements.iter_mut() {
        let words: Vec<String> = Regex::new(r"[^a-zA-Z0-9-']")
            .unwrap()
            .split(&element.to_owned())
            .map(|x| x.to_string())
            .collect();
        *element = words[0].clone();
    }

    // Create Schedule Hashmap
    let cols = number_of_dates + 5;
    let rows = elements.len() / cols;
    let mut schedule: HashMap<String, Vec<f64>> = HashMap::new();

    for r in 0..(rows - 1) {
        let mut qtys: Vec<f64> = Vec::new();
        let mut part_num: String = String::new();
        // let vec3: Vec<_> = vec1.iter().zip(vec2.iter()).map(|(a, b)| a + b).collect();
        for c in 0..cols {
            if c == 0 {
                part_num = elements[cols * r + c].clone();
            }
            if c > 4 {
                qtys.push(elements[cols * r + c].clone().parse().unwrap());
            }
        }

        // If part number is already on the map, add new & existing quantities
        if let Some(existing_qtys) = schedule.get(&part_num) {
            qtys = qtys
                .iter()
                .zip(existing_qtys.iter())
                .map(|(a, b)| a + b)
                .collect();
        }
        schedule.insert(part_num, qtys);
    }

    // Convert Hashmap to JSON & write to disk
    let schedule_json = serde_json::to_string(&schedule).unwrap();
    let mut file = File::create("schedule.json").expect("Could not create file!");
    file.write_all(schedule_json.as_bytes())
        .expect("Cannot write to the file!");

    return Ok(());
}
//
