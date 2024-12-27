use crate::y2015::Y2015;
use crate::y2022::Y2022;
use crate::y2023::Y2023;
use crate::years::{Days, YearSolutions};
use std::collections::HashMap;

pub struct AdventOfCode {
    pub(crate) years: HashMap<u32, Box<dyn YearSolutions>>,
}

impl AdventOfCode {
    pub fn new() -> Self {
        let mut aoc = Self {
            years: HashMap::new(),
        };
        aoc.add_year(2015, Box::new(Y2015));
        aoc.add_year(2022, Box::new(Y2022));
        aoc.add_year(2023, Box::new(Y2023));

        aoc
    }

    pub(crate) fn add_year(&mut self, year: u32, solutions: Box<dyn YearSolutions>) {
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
