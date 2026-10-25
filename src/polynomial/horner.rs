// =============================================================
// PURPOSE: HORNER
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================

// =============================================================
// CORE LOGIC & FUNCTIONS
// =============================================================
pub fn horner_eval(coeffs: &[f64], c: f64) -> Option<f64> {
    if coeffs.is_empty() {
        return None;
    }
    let mut b = coeffs[0];
    for &a in &coeffs[1..] {
        b = b * c + a;
    }
    Some(b)
}

pub fn horner_divide(coeffs: &[f64], c: f64) -> Option<(Vec<f64>, f64)> {
    if coeffs.is_empty() {
        return None;
    }
    let mut b: f64 = coeffs[0];
    let mut result: Vec<f64> = Vec::new();
    result.push(b);
    for &a in &coeffs[1..] {
        b = a + c * b;
        result.push(b);
    }

    let r: f64 = result.pop().unwrap();
    // println!(
    //     "  [Horner Divide] Chia cho c = {}, Thương tạm thời: {:?}, Phần dư r = {}",
    //     c, result, r
    // );
    Some((result, r))
}

pub fn horner_derivative(coeffs: &[f64], c: f64, k: usize) -> Option<f64> {
    if coeffs.is_empty() {
        return None;
    }

    if k >= coeffs.len() {
        return Some(0.0);
    }

    let mut current_coeffs = coeffs.to_vec();
    let mut last_remainder = 0.0;

    for _step in 0..=k {
        let (quotient, remainder) = horner_divide(&current_coeffs, c)?;
        current_coeffs = quotient;
        last_remainder = remainder;
        println!(
            "  [Horner Derivative] Đạo hàm bước {} tại c = {}, phần dư = {}",
            _step, c, last_remainder
        );
    }

    let mut fact = 1.0;
    for i in 1..=k {
        fact *= i as f64;
    }

    let final_val = last_remainder * fact;
    println!(
        "  [Horner Derivative] Kết quả đạo hàm cấp {} tại c = {} là {}",
        k, c, final_val
    );
    Some(final_val)
}

pub fn multiply_poly_with_linear(coeffs: &[f64], c: f64) -> Option<Vec<f64>> {
    if coeffs.is_empty() {
        return None;
    }

    let n = coeffs.len();
    let mut result = vec![0.0; n + 1];

    result[0] = coeffs[0];

    for i in 1..n {
        result[i] = coeffs[i] - c * coeffs[i - 1];
    }

    result[n] = -c * coeffs[n - 1];

    // println!(
    //     "  [Multiply Linear] Nhân đa thức {:?} với (x - {}), kết quả: {:?}",
    //     coeffs, c, result
    // );
    Some(result)
}

pub fn compute_product_polynomial(roots: &[f64]) -> Option<Vec<f64>> {
    if roots.is_empty() {
        return None;
    }

    let mut coeffs = vec![1.0];

    for (idx, &c) in roots.iter().enumerate() {
        coeffs = multiply_poly_with_linear(&coeffs, c)?;
        println!(
            "  [Product Poly] Sau khi nhân nghiệm thứ {} (giá trị {}): {:?}",
            idx + 1,
            c,
            coeffs
        );
    }

    Some(coeffs)
}
