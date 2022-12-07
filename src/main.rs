pub mod utils;
mod y2022;

fn main() {
    let run_2022 = true;

    // 2022
    if run_2022 {
        // Day 1
        let d1_input = utils::read_input(2022, 1);

        let d1_p1 = y2022::day_1::part_1(d1_input.clone());
        println!("2022 Day 01 - P1 - {d1_p1}");

        let d1_p2 = y2022::day_1::part_2(d1_input);
        println!("2022 Day 01 - P2 - {d1_p2}");
    }
}
