use crate::y2022::Y2022;
use std::collections::HashMap;

pub struct Days {
    pub all: bool,
    pub days: Vec<u32>,
}

pub trait Day {
    fn run(&self);
}

pub trait YearSolutions {
    fn get_day(&self, day: u32) -> Option<Box<dyn Day>>;
    fn all_days(&self) -> Vec<Box<dyn Day>> {
        (1..=25).filter_map(|d| self.get_day(d)).collect()
    }
}

pub struct AdventOfCode {
    years: HashMap<u32, Box<dyn YearSolutions>>,
}

impl AdventOfCode {
    pub fn new() -> Self {
        let mut aoc = Self {
            years: HashMap::new(),
        };
        aoc.add_year(2022, Box::new(Y2022));

        aoc
    }

    fn add_year(&mut self, year: u32, solutions: Box<dyn YearSolutions>) {
        self.years.insert(year, solutions);
    }

    pub fn run_all_years(&self) {
        let mut years: Vec<&u32> = self.years.keys().collect();
        years.sort();
        for year in years {
            self.run_year(
                *year,
                &Days {
                    all: true,
                    days: vec![],
                },
            )
        }
    }

    pub fn run_year(&self, year: u32, days: &Days) {
        if let Some(year_solutions) = self.years.get(&year) {
            println!("\nYear {}", year);
            if days.all {
                for day in year_solutions.all_days() {
                    day.run();
                }
            } else {
                for &day in &days.days {
                    if let Some(day_solution) = year_solutions.get_day(day) {
                        day_solution.run();
                    } else {
                        println!("Day {} not implemented for year {}", day, year);
                    }
                }
            }
        } else {
            println!("Year {} not implemented", year);
        }
    }
}
