use fs_err as fs;
use fs_err::{File, OpenOptions};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header;
use serde_json::value::{to_value, Value};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use tera::{try_get_value, Context, Error, Result, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => panic!("Parsing error(s): {}", e),
        };
        tera.register_filter("zero_pad", zero_pad);
        tera
    };
}

pub fn read_input(year: u32, day: u32) -> Vec<String> {
    let filename = format!("./inputs/{year}/day_{day}.txt");

    let path = std::path::Path::new(&filename);

    let contents = if path.exists() {
        match fs::read_to_string(&filename) {
            Ok(contents) => contents,
            Err(e) => panic!("no such file '{filename}' - {e}"),
        }
    } else {
        println!("Input for Year: {year}, Day: {day} does not exist");
        println!("Downloading input");
        download_input(year, day)
    };

    let lines = contents.lines().map(|s| s.to_owned()).collect::<Vec<_>>();

    lines
}

pub fn download_input(year: u32, day: u32) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session = get_session();
    // cookies = {"session": session}

    let mut headers = header::HeaderMap::new();
    headers.insert(
        reqwest::header::COOKIE,
        header::HeaderValue::from_str(&format!("session={session}")).unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let res = client.get(url).send().unwrap();

    let body = res.text().unwrap();

    let path = format!("./inputs/{year}");
    let filename = format!("{path}/day_{day}.txt");
    fs::create_dir_all(path).unwrap();
    let mut file = fs::File::create(filename).unwrap();
    file.write_all(body.as_bytes()).unwrap();

    body
}

fn get_session() -> String {
    let filename = "./.session";
    match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("please put your session cookie in '{filename}' - {e}"),
    }
}

pub fn zero_pad(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("zero_pad", "value", u32, value);
    let width = match args.get("width") {
        Some(val) => try_get_value!("zero_pad", "width", usize, val),
        None => {
            return Err(Error::msg(
                "Filter `zero_pad` expected an arg called `width`",
            ))
        }
    };

    Ok(to_value(format!("{s:0width$}", width = width)).unwrap())
}

pub fn create_year_day(year: u32, day: u32) {
    let mut context = Context::new();
    context.insert("year", &year);
    context.insert("day", &day);

    // Create day file
    let day_content = match TEMPLATES.render("day.rs", &context) {
        Ok(s) => s,
        Err(e) => panic!("Error rendering day template: {e}"),
    };

    let filepath = format!("./src/y{year}");
    let filename = format!("{filepath}/day_{day}.rs");

    if !Path::new(&filename).exists() {
        println!("Creating {filename}");
        fs::create_dir_all(&filepath).unwrap();
        let mut file = File::create(&filename).unwrap();
        file.write_all(day_content.as_bytes()).unwrap();
    }

    // Update or create mod.rs file
    let mod_file = format!("{filepath}/mod.rs");
    if Path::new(&mod_file).exists() {
        update_mod_file(&mod_file, day);
    } else {
        create_mod_file(&mod_file, year, day);
    }
}

fn update_mod_file(mod_file: &str, day: u32) {
    println!("Updating {mod_file}");
    let mut content = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(mod_file)
        .unwrap();

    file.read_to_string(&mut content).unwrap();

    // Add new pub mod line if it doesn't exist
    let new_mod_line = format!("pub mod day_{day};");
    if !content.contains(&new_mod_line) {
        // Find the last pub mod line and insert after it
        let re = Regex::new(r"(?m)^pub mod day_\d+;$").unwrap();
        if let Some(last_match) = re.find_iter(&content).last() {
            let insert_pos = last_match.end();
            content.insert_str(insert_pos, &format!("\n{new_mod_line}"));
        } else {
            // If no existing pub mod lines, insert at the beginning
            content = format!("{new_mod_line}\n{content}");
        }
    }

    // Update the match expression in get_day
    let re = Regex::new(r"match day \{([\s\S]*?)_ => None,").unwrap();
    if let Some(captures) = re.captures(&content) {
        let mut match_content = captures.get(1).unwrap().as_str().to_string();
        let new_match_line = format!("            {day} => Some(Box::new(day_{day}::AocDay)),\n");
        if !match_content.contains(&new_match_line) {
            match_content.push_str(&new_match_line);
            // Sort the match arms
            let mut arms: Vec<String> = match_content
                .lines()
                .filter(|line| line.contains("=>"))
                .map(|line| line.trim().to_string())
                .collect();
            arms.sort_by(|a, b| {
                let a_num: u32 = a.split_whitespace().next().unwrap().parse().unwrap();
                let b_num: u32 = b.split_whitespace().next().unwrap().parse().unwrap();
                a_num.cmp(&b_num)
            });
            let sorted_match_content = arms.join("\n            ");
            content = re
                .replace(
                    &content,
                    format!(
                        "match day {{\n            {sorted_match_content}\n            _ => None,"
                    ),
                )
                .into_owned();
        }
    }

    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.set_len(content.len() as u64).unwrap();
}

fn create_mod_file(mod_file: &str, year: u32, day: u32) {
    println!("Creating {mod_file}");
    let mut context = Context::new();
    context.insert("year", &year);
    context.insert("day", &day);

    let content = match TEMPLATES.render("mod.rs", &context) {
        Ok(s) => s,
        Err(e) => panic!("Error rendering mod template: {e}"),
    };

    let mut file = File::create(mod_file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
