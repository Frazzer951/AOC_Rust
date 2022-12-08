use std::io::Write;
use std::path::Path;

use fs_err as fs;
use fs_err::OpenOptions;
use reqwest::header;

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

pub fn create_year_day(year: u32, day: u32) {
    let mut template = match fs::read_to_string("./templates/template.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{e}"),
    };
    template = template.replace("{{year}}", &format!("{year}"));
    template = template.replace("{{day}}", &format!("{day}"));
    template = template.replace("{{day_02}}", &format!("{day:0width$}", width = 2));

    let filepath = format!("./src/y{year}");
    let filename = format!("{filepath}/day_{day}.rs");

    if !Path::new(&filename).exists() {
        // Create day file
        println!("Creating {filename}");
        fs::create_dir_all(filepath).unwrap();
        let mut file = fs::File::create(filename).unwrap();
        file.write_all(template.as_bytes()).unwrap();

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
