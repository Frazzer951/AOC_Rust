pub mod utils;
mod y2022;

fn main() {
    let run_2022 = true;

    // 2022
    if run_2022 {
        // Day 1
        y2022::day_1::run();

        // Day 2
        y2022::day_2::run();
    }
}
