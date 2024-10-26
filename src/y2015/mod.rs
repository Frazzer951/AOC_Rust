pub mod day_1;

use crate::years::{Day, YearSolutions};

pub struct Y2015;

impl YearSolutions for Y2015 {
    fn get_day(&self, day: u32) -> Option<Box<dyn Day>> {
        match day {
            1 => Some(Box::new(day_1::AocDay)),
            _ => None,
        }
    }
}
