pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;

use crate::years::{Day, YearSolutions};

pub struct Y2022;

impl YearSolutions for Y2022 {
    fn get_day(&self, day: u32) -> Option<Box<dyn Day>> {
        match day {
            1 => Some(Box::new(day_1::AocDay)),
            2 => Some(Box::new(day_2::AocDay)),
            3 => Some(Box::new(day_3::AocDay)),
            4 => Some(Box::new(day_4::AocDay)),
            5 => Some(Box::new(day_5::AocDay)),
            6 => Some(Box::new(day_6::AocDay)),
            7 => Some(Box::new(day_7::AocDay)),
            8 => Some(Box::new(day_8::AocDay)),
            _ => None,
        }
    }
}
