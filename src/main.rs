// =============================================================
// PURPOSE: MAIN ENDPOINT
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use pps::core::io::read_txt_points;
use pps::polynomial::central::extract_central_points;
use pps::polynomial::horner::{horner_derivative, horner_eval};
use pps::polynomial::newton::newton_interpolation;
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
    let result = extract_central_points(points, 3.52, 9).unwrap();

    let newton = newton_interpolation(&result).unwrap();

    let f = horner_eval(&newton, 3.52).unwrap();
    let f_1 = horner_derivative(&newton, 3.52, 1).unwrap();
    println!("{}", f);
    println!("=================================================");
    println!("{}", f_1);
}
