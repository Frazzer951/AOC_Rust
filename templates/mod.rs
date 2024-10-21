pub mod day_{{ day }};

use crate::years::{Day, YearSolutions};

pub struct Y{{ year }};

impl YearSolutions for Y{{ year }} {
    fn get_day(&self, day: u32) -> Option<Box<dyn Day>> {
        match day {
            {{ day }} => Some(Box::new(day_{{ day }}::AocDay)),
            _ => None,
        }
    }
}
