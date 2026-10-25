// =============================================================
// PURPOSE:
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use crate::polynomial::horner::compute_product_polynomial;

// =============================================================
// CORE LOGIC & FUNCTIONS
// =============================================================

// NEWTON: FORWARD
pub fn newton_interpolation(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.is_empty() {
        return None;
    }

    let n = points.len();
    let x_vals: Vec<f64> = points.iter().map(|&(x, _)| x).collect();
    let y_vals: Vec<f64> = points.iter().map(|&(_, y)| y).collect();

    // 1. Xây dựng bảng tỷ hiệu (divided difference table) đầy đủ để in
    // table[i][j] trong đó i là chỉ số hàng, j là bậc tỷ hiệu
    let mut table = vec![vec![0.0; n]; n];
    for i in 0..n {
        table[i][0] = y_vals[i];
    }

    for j in 1..n {
        for i in 0..(n - j) {
            let numerator = table[i + 1][j - 1] - table[i][j - 1];
            let denominator = x_vals[i + j] - x_vals[i];
            if denominator.abs() < 1e-12 {
                return None; // Tránh chia cho 0 nếu trùng mốc
            }
            table[i][j] = numerator / denominator;
        }
    }

    // In bảng tỷ sai phân (tỷ hiệu) ra màn hình
    println!("\n================ BẢNG TỶ SAI PHÂN (TỶ HIỆU) ================");
    print!("{:>8} | {:>8}", "x_i", "y_i");
    for j in 1..n {
        print!(" | f[0..{}]", j);
    }
    println!();
    println!("{}", "-".repeat(12 + 14 * n));

    for i in 0..n {
        print!("{:8.4} | {:8.4}", x_vals[i], table[i][0]);
        for j in 1..n {
            if i <= n - 1 - j {
                print!(" | {:10.4}", table[i][j]);
            } else {
                print!(" | {:>10}", "");
            }
        }
        println!();
    }
    println!("============================================================");

    // Lấy các hệ số Newton chính là đường chéo của bảng (table[0][i])
    let mut newton_coeffs = Vec::new();
    for i in 0..n {
        newton_coeffs.push(table[0][i]);
    }

    println!("\n--- DANH SÁCH CÁC HỆ SỐ NEWTON (a_i) ---");
    for (i, &coeff) in newton_coeffs.iter().enumerate() {
        println!("  a_{} = f[x0, ..., x{}] = {:.6}", i, i, coeff);
    }

    // 2. Xây dựng đa thức chính tắc và in quá trình từ đầu vòng lặp đến cuối
    println!("\n--- QUÁ TRÌNH XÂY DỰNG ĐA THỨC QUA TỪNG BƯỚC ---");
    let mut result = vec![0.0];

    for i in 0..n {
        let a_i = newton_coeffs[i];

        let term_poly: Vec<f64> = if i == 0 {
            vec![1.0] // Đa thức cơ sở hằng số 1 cho bước đầu a_0
        } else {
            let roots = &x_vals[0..i];
            compute_product_polynomial(roots)?
        };

        // Nhân hệ số a_i vào đa thức tích cơ sở
        let scaled_term: Vec<f64> = term_poly.iter().map(|&coeff| coeff * a_i).collect();

        println!("Bước i = {}:", i);
        println!("  + Hệ số a_{} = {:.4}", i, a_i);
        println!("  + Số hạng tích lũy bậc {}: {:?}", i, scaled_term);

        // Căn chỉnh bậc và cộng dồn vào đa thức tổng kết quả
        let max_len = std::cmp::max(result.len(), scaled_term.len());
        let mut new_result = vec![0.0; max_len];
        let mut padded_term = vec![0.0; max_len];

        let diff_res = max_len - result.len();
        for (idx, &val) in result.iter().enumerate() {
            new_result[idx + diff_res] = val;
        }

        let diff_term = max_len - scaled_term.len();
        for (idx, &val) in scaled_term.iter().enumerate() {
            padded_term[idx + diff_term] = val;
        }

        for j in 0..max_len {
            new_result[j] += padded_term[j];
        }
        result = new_result;
        println!("  + Đa thức P(x) tạm thời: {:?}", result);
        println!("------------------------------------------------------------");
    }

    Some(result)
}

// NEWTON: BACKWARD

pub fn newton_backward_interpolation(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    if points.is_empty() {
        return None;
    }

    let n = points.len();
    let mut rev_points = points.to_vec();
    rev_points.reverse();

    let x_vals: Vec<f64> = rev_points.iter().map(|&(x, _)| x).collect();
    let y_vals: Vec<f64> = rev_points.iter().map(|&(_, y)| y).collect();

    let mut table = vec![vec![0.0; n]; n];
    for i in 0..n {
        table[i][0] = y_vals[i];
    }

    for j in 1..n {
        for i in 0..(n - j) {
            let numerator = table[i + 1][j - 1] - table[i][j - 1];
            let denominator = x_vals[i + j] - x_vals[i];
            if denominator.abs() < 1e-12 {
                return None;
            }
            table[i][j] = numerator / denominator;
        }
    }

    println!("\n================ BẢNG TỶ SAI PHÂN LÙI (TỶ HIỆU) ================");
    print!("{:>8} | {:>8}", "x_i", "y_i");
    for j in 1..n {
        print!(" | f[n..{}]", n - 1 - j);
    }
    println!();
    println!("{}", "-".repeat(12 + 14 * n));

    for i in 0..n {
        print!("{:8.4} | {:8.4}", x_vals[i], table[i][0]);
        for j in 1..n {
            if i <= n - 1 - j {
                print!(" | {:10.4}", table[i][j]);
            } else {
                print!(" | {:>10}", "");
            }
        }
        println!();
    }
    println!("================================================================");

    let mut newton_coeffs = Vec::new();
    for i in 0..n {
        newton_coeffs.push(table[0][i]);
    }

    println!("\n--- DANH SÁCH CÁC HỆ SỐ NEWTON LÙI (a_i) ---");
    for (i, &coeff) in newton_coeffs.iter().enumerate() {
        println!("  a_{} = {:.6}", i, coeff);
    }

    println!("\n--- QUÁ TRÌNH XÂY DỰNG ĐA THỨC LÙI QUA TỪNG BƯỚC ---");
    let mut result = vec![0.0];

    for i in 0..n {
        let a_i = newton_coeffs[i];

        let term_poly: Vec<f64> = if i == 0 {
            vec![1.0]
        } else {
            let roots = &x_vals[0..i];
            compute_product_polynomial(roots)?
        };

        let scaled_term: Vec<f64> = term_poly.iter().map(|&coeff| coeff * a_i).collect();

        println!("Bước i = {}:", i);
        println!("  + Hệ số a_{} = {:.4}", i, a_i);
        println!("  + Số hạng tích lũy bậc {}: {:?}", i, scaled_term);

        let max_len = std::cmp::max(result.len(), scaled_term.len());
        let mut new_result = vec![0.0; max_len];
        let mut padded_term = vec![0.0; max_len];

        let diff_res = max_len - result.len();
        for (idx, &val) in result.iter().enumerate() {
            new_result[idx + diff_res] = val;
        }

        let diff_term = max_len - scaled_term.len();
        for (idx, &val) in scaled_term.iter().enumerate() {
            padded_term[idx + diff_term] = val;
        }

        for j in 0..max_len {
            new_result[j] += padded_term[j];
        }
        result = new_result;
        println!("  + Đa thức P(x) tạm thời: {:?}", result);
        println!("------------------------------------------------------------");
    }

    Some(result)
}
