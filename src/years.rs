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
