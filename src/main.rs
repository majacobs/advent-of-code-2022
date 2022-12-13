mod common;
mod harness;
mod year2015;
mod year2022;

fn main() {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("2022"));
    match mode.as_str() {
        "2015" => year2015::run_all(),
        "2022" => year2022::run_all(),
        "all" => {
            year2015::run_all();
            year2022::run_all();
        }
        _ => eprintln!("Invalid arg"),
    }
}
