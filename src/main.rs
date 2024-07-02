use std::env;

mod workflows;
mod _01_rna;
mod _02_revc;
mod _03_scsp;

fn main() {
    let selected_problem = env::args().last().unwrap_or_default();
    match selected_problem.as_str() {
        "1" => _01_rna::run(),
        "2" =>_02_revc::run(),
        "3" => _03_scsp::run(),
        _ => _03_scsp::run()
    }
    
}
