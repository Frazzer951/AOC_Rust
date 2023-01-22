use crate::y2022;

pub struct Years {
    pub years: Vec<Year>,
}

impl Years {
    pub fn new() -> Self {
        Self { years: vec![] }
    }

    pub fn add_year(mut self, year: Year) -> Self {
        self.years.push(year);
        self
    }
}

pub struct Year {
    pub year: u32,
    pub days: Days,
}

impl Year {
    pub fn new(year: u32) -> Self {
        Self { year, days: Days::new() }
    }

    pub fn set_day(mut self, day: u32) -> Self {
        self.days.add_day(day);
        self
    }
}

pub struct Days {
    pub all: bool,
    pub days: Vec<u32>,
}

impl Days {
    pub fn new() -> Self {
        Self { all: true, days: vec![] }
    }

    pub fn add_day(&mut self, day: u32) {
        self.all = false;
        self.days.push(day);
    }
}

pub fn run_years(years: Years) {
    for year in years.years {
        match year.year {
            2022 => y2022(year.days),
            year => {
                println!("Year `{year}` hasn't been setup yet")
            },
        }
    }
}

pub fn y2022(days: Days) {
    println!("2022");
    if days.all || days.days.contains(&1) {
        y2022::day_1::run();
    }
    if days.all || days.days.contains(&2) {
        y2022::day_2::run();
    }
    if days.all || days.days.contains(&3) {
        y2022::day_3::run();
    }
    if days.all || days.days.contains(&4) {
        y2022::day_4::run();
    }
    if days.all || days.days.contains(&5) {
        y2022::day_5::run();
    }
    if days.all || days.days.contains(&6) {
        y2022::day_6::run();
    }
    if days.all || days.days.contains(&7) {
        y2022::day_7::run();
    }
    if days.all || days.days.contains(&8) {
        y2022::day_8::run();
    }
}
