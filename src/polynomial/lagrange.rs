// =============================================================
// PURPOSE: LAGRANGE PDE
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use crate::polynomial::horner::{compute_product_polynomial, horner_divide, horner_eval};
// =============================================================
// PARAMETERS
// =============================================================

// =============================================================
// CORE LOGIC & FUNCTIONS
// =============================================================

pub fn lagrange(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.is_empty() {
        return None;
    }

    let x_vals: Vec<f64> = points.iter().map(|&(x, _)| x).collect();
    let w_poly = compute_product_polynomial(&x_vals)?;

    let len = points.len();
    let mut final_coeffs: Vec<f64> = Vec::new();

    for i in 0..len {
        let (xi, yi) = points[i];

        let (quotient, _remainder) = horner_divide(&w_poly, xi)?;

        let d_i = horner_eval(&quotient, xi)?;

        if d_i.abs() < 1e-12 {
            return None;
        }

        let scale = yi / d_i;
        let scaled_l_i: Vec<f64> = quotient.iter().map(|&coeff| coeff * scale).collect();

        if final_coeffs.is_empty() {
            final_coeffs = scaled_l_i;
        } else {
            let max_len = std::cmp::max(final_coeffs.len(), scaled_l_i.len());
            let mut new_coeffs = vec![0.0; max_len];

            let diff_a = max_len - final_coeffs.len();
            let diff_b = max_len - scaled_l_i.len();

            for j in 0..max_len {
                let val_a = if j >= diff_a {
                    final_coeffs[j - diff_a]
                } else {
                    0.0
                };
                let val_b = if j >= diff_b {
                    scaled_l_i[j - diff_b]
                } else {
                    0.0
                };
                new_coeffs[j] = val_a + val_b;
            }
            final_coeffs = new_coeffs;
        }
    }

    Some(final_coeffs)
}
