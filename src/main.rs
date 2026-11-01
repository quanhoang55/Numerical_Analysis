// =============================================================
// PURPOSE: MAIN ENDPOINT
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use pps::core::io::read_txt_points;
use pps::polynomial::central::stirling_coefficients;
use pps::polynomial::horner::horner_eval;
// =============================================================
// PARAMETERS
// =============================================================

// =============================================================
// CORE LOGIC & FUNCTIONS
// =============================================================

// =============================================================
// MAIN
// =============================================================
fn main() {
    let txt_path = "data/11b.txt";

    let output_txt = read_txt_points(txt_path).expect("empty");
    let points: &[(f64, f64)] = &output_txt;

    let result = stirling_coefficients(points, 0.1);
    println!("{:?}", result);

    println!("======================================================");
    let f_1 = horner_eval(&result, 9.45).unwrap();
    println!("f_1: {}", f_1);
}
