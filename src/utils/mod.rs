use std::io::Write;

use fs_err as fs;
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

    let path = format!("./inputs/{year}/");
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
