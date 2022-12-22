use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use fs_err as fs;
use fs_err::OpenOptions;
use lazy_static::lazy_static;
use reqwest::header;
use serde_json::value::{to_value, Value};
use tera::{try_get_value, Context, Error, Result, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            },
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

    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();
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
        None => return Err(Error::msg("Filter `zero_pad` expected an arg called `width`")),
    };

    Ok(to_value(format!("{s:0width$}", width = width)).unwrap())
}

pub fn create_year_day(year: u32, day: u32) {
    let mut context = Context::new();
    context.insert("year", &year);
    context.insert("day", &day);

    let content = match TEMPLATES.render("day.rs", &context) {
        Ok(s) => s,
        Err(e) => panic!("Error: {e}"),
    };

    let filepath = format!("./src/y{year}");
    let filename = format!("{filepath}/day_{day}.rs");

    if !Path::new(&filename).exists() {
        // Create day file
        println!("Creating {filename}");
        fs::create_dir_all(filepath).unwrap();
        let mut file = fs::File::create(filename).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        // Append mod to mod.rs file.
        let mod_file = format!("./src/y{year}/mod.rs");
        println!("Appending to {mod_file}");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(mod_file)
            .unwrap();

        if let Err(e) = writeln!(file, "pub mod day_{day};") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
