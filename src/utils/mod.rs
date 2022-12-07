use fs_err as fs;

pub fn read_input(year: u32, day: u32) -> Vec<String> {
    let filename = format!("./inputs/{year}/day_{day}.txt");

    let contents = match fs::read_to_string(&filename) {
        Ok(contents) => contents,
        Err(e) => panic!("no such file '{filename}' - {e}"),
    };

    let lines = contents.lines().map(|s| s.to_owned()).collect::<Vec<_>>();

    lines
}
