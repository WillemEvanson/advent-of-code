mod solution;

pub mod bit_set;
pub mod direction;
pub mod grid;
pub mod math;
pub mod rng;

pub use solution::{Answer, Solution};

pub fn run_year(binary_name: &str, functions: &[fn(&str) -> Solution]) {
    let Some(input_dir) = std::env::args_os().nth(1) else {
        eprintln!("Usage: {binary_name} [input directory]");
        return;
    };

    let path = std::path::Path::new(&input_dir);
    let mut total_duration = std::time::Duration::default();
    for i in 1..=functions.len() {
        let path = path.join(format!("day{i:02}.txt"));
        if let Ok(input) = std::fs::read_to_string(&path) {
            let now = std::time::Instant::now();
            let solution = functions[i - 1](&input);
            let elapsed = now.elapsed();
            total_duration += now.elapsed();

            println!("Day {i}:\t{solution}{elapsed:?}");
        }
    }
    println!("Total:\t{:<80}{total_duration:?}", "");
}
