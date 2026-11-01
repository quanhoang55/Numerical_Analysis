// =============================================================
// PURPOSE: GAUSS 1, GAUSS 2, STERLING, BESSEL
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================

// =============================================================
// CORE LOGIC & FUNCTIONS
// =============================================================

/// Trích xuất k điểm nội suy từ danh sách cách đều sao cho điểm x_bar nằm ở vùng trung tâm.
/// - points: Danh sách toàn bộ các điểm (x, y) ban đầu (đã sắp xếp tăng dần theo x).
/// - x_bar: Điểm cần tính gần đúng.
/// - k: Số lượng điểm cần trích xuất (thường là số lẻ cho Gauss 1 / Stirling, hoặc số chẵn cho Gauss 2 / Bessel).
pub fn extract_central_points(
    points: &[(f64, f64)],
    x_bar: f64,
    k: usize,
) -> Option<Vec<(f64, f64)>> {
    if points.is_empty() || k == 0 || k > points.len() {
        return None;
    }

    let n = points.len();

    // Bước 1: Tìm mốc x_i trong danh sách có khoảng cách tới x_bar là nhỏ nhất (mốc trung tâm)
    let mut closest_idx = 0;
    let mut min_diff = (points[0].0 - x_bar).abs();

    for i in 1..n {
        let diff = (points[i].0 - x_bar).abs();
        if diff < min_diff {
            min_diff = diff;
            closest_idx = i;
        }
    }

    // Bước 2: Xác định khoảng cách lấy mốc sang trái (left) và sang phải (right) từ closest_idx
    // Để cân đối k điểm:
    // Nếu k lẻ, số điểm hai bên có thể lệch nhau 1 đơn vị tùy thuộc vào x_bar nằm lệch trái hay phải.
    let half_k = k / 2;
    let mut start_idx;
    let mut end_idx;

    if x_bar >= points[closest_idx].0 {
        // Nếu x_bar nằm bên phải hoặc bằng mốc tâm, ưu tiên lấy rộng hơn về phía bên phải
        start_idx = closest_idx.checked_sub(half_k).unwrap_or(0);
        end_idx = start_idx + k - 1;

        // Kiểm soát tràn biên phía trên
        if end_idx >= n {
            end_idx = n - 1;
            start_idx = end_idx.checked_sub(k - 1).unwrap_or(0);
        }
    } else {
        // Nếu x_bar nằm bên trái mốc tâm, ưu tiên lấy rộng hơn về phía bên trái
        end_idx = std::cmp::min(closest_idx + half_k, n - 1);
        start_idx = end_idx.checked_sub(k - 1).unwrap_or(0);

        // Kiểm soát tràn biên phía dưới
        if start_idx == 0 && (end_idx - start_idx + 1) < k {
            end_idx = std::cmp::min(k - 1, n - 1);
        }
    }

    // Bước 3: Trích xuất tập con k điểm
    let extracted: Vec<(f64, f64)> = points[start_idx..=end_idx].to_vec();

    // println!("\n[Trích xuất mốc nội suy]");
    // println!("  - Điểm cần tính x_bar = {}", x_bar);
    // println!("  - Mốc trung tâm gần nhất: x = {}", points[closest_idx].0);
    // println!("  - Số lượng điểm yêu cầu: k = {}", k);
    // println!("  - Các mốc trích xuất được: {:?}", extracted.iter().map(|p| p.0).collect::<Vec<_>>());

    Some(extracted)
}

// ============================================================================
// GAUSS 1
// ============================================================================

/// Hàm tính giai thừa trả về kiểu f64
fn factorial(n: usize) -> f64 {
    (1..=n).fold(1.0, |acc, x| acc * (x as f64))
}

/// Thuật toán tính hệ số đa thức nội suy Gauss I (có in debug trực quan)
pub fn gauss_i_coefficients(points: &[(f64, f64)], h: f64) -> Vec<f64> {
    let k = points.len();
    assert!(k % 2 == 1, "Số lượng mốc k phải là số lẻ!");

    let idx = k / 2;
    let mut delta = vec![vec![0.0; k]; k];

    println!("\n=== [DEBUG] THÔNG TIN ĐẦU VÀO ===");
    println!("Số lượng mốc k = {}, Bước lưới h = {}", k, h);
    println!(
        "Mốc tâm (x_0, y_0) ở chỉ số idx = {} -> x = {}, y = {}",
        idx, points[idx].0, points[idx].1
    );

    // Bước 1: Khởi tạo cột 0 và tính bảng sai phân tiến
    for i in 0..k {
        delta[i][0] = points[i].1;
    }

    for col in 1..k {
        for row in 0..(k - col) {
            delta[row][col] = delta[row + 1][col - 1] - delta[row][col - 1];
        }
    }

    // In trực quan ma trận delta
    println!("\n=== [DEBUG] BẢNG SAI PHÂN TIẾN (delta) ===");
    print!("{:>6} |", "row\\col");
    for col in 0..k {
        print!(" {:>10} |", format!("col {}", col));
    }
    println!("\n{:-<70}", "");

    for row in 0..k {
        print!("{:>6} |", row);
        for col in 0..k {
            if col < k - row {
                print!(" {:10.4} |", delta[row][col]);
            } else {
                print!(" {:>10} |", "");
            }
        }
        println!();
    }

    // Bước 2: Trích xuất hệ số theo đường đi Gauss I
    let mut coeff = vec![0.0; k];
    coeff[0] = delta[idx][0]; // Hệ số ứng với y_0

    println!("\n=== [DEBUG] TRÍCH XUẤT HỆ SỐ GAUSS I (Thô) ===");
    println!("m = 0 (y_0): delta[{}][0] = {:.4}", idx, coeff[0]);

    for m in 1..k {
        let r = if m % 2 == 0 {
            idx - m / 2
        } else {
            idx - (m - 1) / 2
        };
        coeff[m] = delta[r][m];
        println!(
            "m = {} (Cấp {}): Lấy từ delta[{}][{}] = {:.4}",
            m, m, r, m, coeff[m]
        );
    }

    // Bước 3: Chuẩn hóa hệ số theo cấp giai thừa và h^m
    let mut h_pow = 1.0;
    for m in 0..k {
        let fact = factorial(m);
        coeff[m] = coeff[m] / (fact * h_pow);
        h_pow *= h;
    }

    println!("\n=== [DEBUG] HỆ SỐ ĐA THỨC CHUẨN HÓA (coeff theo t) ===");
    for (m, &val) in coeff.iter().enumerate() {
        println!("a_{} = {:.6}", m, val);
    }
    println!("==================================================\n");

    coeff
}

// ============================================================================
// GAUSS 2
// ============================================================================

// ============================================================================
// STIRLING
// ============================================================================

/// Thuật toán tính hệ số đa thức nội suy Stirling (có in debug trực quan qua từng bước)
pub fn stirling_coefficients(points: &[(f64, f64)], h: f64) -> Vec<f64> {
    let k = points.len();
    assert!(k % 2 == 1, "Số lượng mốc k phải là số lẻ!");

    let idx = k / 2;
    let mut delta = vec![vec![0.0; k]; k];

    println!("\n=== [DEBUG] 1. THÔNG TIN ĐẦU VÀO & BẢNG SAI PHÂN TIẾN ===");
    println!("Số lượng mốc k = {}, Bước lưới h = {}", k, h);
    println!(
        "Mốc tâm (x_0, y_0) ở chỉ số idx = {} -> x = {}, y = {}",
        idx, points[idx].0, points[idx].1
    );

    // Bước 1: Khởi tạo cột 0 và tính bảng sai phân tiến
    for i in 0..k {
        delta[i][0] = points[i].1;
    }

    for col in 1..k {
        for row in 0..(k - col) {
            delta[row][col] = delta[row + 1][col - 1] - delta[row][col - 1];
        }
    }

    // In trực quan ma trận delta dạng bảng
    print!("{:>6} |", "row\\col");
    for col in 0..k {
        print!(" {:>10} |", format!("col {}", col));
    }
    println!("\n{:-<70}", "");

    for row in 0..k {
        print!("{:>6} |", row);
        for col in 0..k {
            if col < k - row {
                print!(" {:10.4} |", delta[row][col]);
            } else {
                print!(" {:>10} |", "");
            }
        }
        println!();
    }

    // Bước 2: Trích xuất hệ số theo đường đi Stirling
    let mut coeff = vec![0.0; k];
    coeff[0] = delta[idx][0]; // Hệ số ứng với y_0

    println!("\n=== [DEBUG] 2. TRÍCH XUẤT HỆ SỐ STIRLING (Thô) ===");
    println!("m = 0 (Cấp 0 - Tâm): delta[{}][0] = {:.4}", idx, coeff[0]);

    for m in 1..k {
        if m % 2 != 0 {
            // Cấp lẻ: Lấy trung bình cộng hai giá trị đối xứng từ hai hướng (Gauss I và Gauss II)
            let r1 = idx - (m - 1) / 2;
            let r2 = idx - (m + 1) / 2;
            coeff[m] = (delta[r1][m] + delta[r2][m]) / 2.0;
            println!(
                "m = {} (Cấp lẻ): Trung bình (delta[{}][{}] = {:.4} + delta[{}][{}] = {:.4}) / 2 = {:.4}",
                m, r1, m, delta[r1][m], r2, m, delta[r2][m], coeff[m]
            );
        } else {
            // Cấp chẵn: Giữ nguyên giá trị tại mốc trung tâm tương ứng
            let r = idx - m / 2;
            coeff[m] = delta[r][m];
            println!(
                "m = {} (Cấp chẵn): Lấy trực tiếp từ mốc trung tâm delta[{}][{}] = {:.4}",
                m, r, m, coeff[m]
            );
        }
    }

    // Bước 3: Chuẩn hóa hệ số theo cấp giai thừa và h^m
    let mut h_pow = 1.0;
    for m in 0..k {
        let fact = factorial(m);
        coeff[m] = coeff[m] / (fact * h_pow);
        h_pow *= h;
    }

    println!("\n=== [DEBUG] 3. HỆ SỐ ĐA THỨC CHUẨN HÓA (coeff theo t) ===");
    for (m, &val) in coeff.iter().enumerate() {
        println!("a_{} = {:.6}", m, val);
    }
    println!("======================================================\n");

    coeff
}

// ============================================================================
// BESSEL
// ============================================================================

/// Thuật toán tính hệ số đa thức nội suy Bessel (cho số mốc k chẵn)
pub fn bessel_coefficients(points: &[(f64, f64)], h: f64) -> Vec<f64> {
    let k = points.len();
    assert!(
        k % 2 == 0,
        "Số lượng mốc k của phương pháp Bessel phải là số chẵn!"
    );

    // Chỉ số mốc trung tâm cho trường hợp chẵn (cặp mốc chính giữa x_0 và x_1)
    let idx = k / 2 - 1;
    let mut delta = vec![vec![0.0; k]; k];

    println!("\n=== [DEBUG] 1. THÔNG TIN ĐẦU VÀO & BẢNG SAI PHÂN TIẾN ===");
    println!("Số lượng mốc k = {}, Bước lưới h = {}", k, h);
    println!(
        "Cặp mốc trung tâm: x_0 ở chỉ số {} (x = {}, y = {}) và x_1 ở chỉ số {} (x = {}, y = {})",
        idx,
        points[idx].0,
        points[idx].1,
        idx + 1,
        points[idx + 1].0,
        points[idx + 1].1
    );

    // Bước 1: Khởi tạo cột 0 và tính bảng sai phân tiến
    for i in 0..k {
        delta[i][0] = points[i].1;
    }

    for col in 1..k {
        for row in 0..(k - col) {
            delta[row][col] = delta[row + 1][col - 1] - delta[row][col - 1];
        }
    }

    // In trực quan ma trận delta dạng bảng
    print!("{:>6} |", "row\\col");
    for col in 0..k {
        print!(" {:>10} |", format!("col {}", col));
    }
    println!("\n{:-<75}", "");

    for row in 0..k {
        print!("{:>6} |", row);
        for col in 0..k {
            if col < k - row {
                print!(" {:10.4} |", delta[row][col]);
            } else {
                print!(" {:>10} |", "");
            }
        }
        println!();
    }

    // Bước 2: Trích xuất hệ số theo đường đi Bessel
    let mut coeff = vec![0.0; k];

    println!("\n=== [DEBUG] 2. TRÍCH XUẤT HỆ SỐ BESSEL (Thô) ===");

    for m in 0..k {
        if m % 2 == 0 {
            // Cấp chẵn: Lấy trung bình cộng giữa hai giá trị đối xứng từ Gauss I và Gauss II
            let r1 = idx - m / 2 + 1;
            let r2 = idx - m / 2;
            coeff[m] = (delta[r1][m] + delta[r2][m]) / 2.0;
            println!(
                "m = {} (Cấp chẵn): Trung bình (delta[{}][{}] = {:.4} + delta[{}][{}] = {:.4}) / 2 = {:.4}",
                m, r1, m, delta[r1][m], r2, m, delta[r2][m], coeff[m]
            );
        } else {
            // Cấp lẻ: Lấy trực tiếp từ mốc trung tâm tương ứng
            let r = idx - (m - 1) / 2;
            coeff[m] = delta[r][m];
            println!(
                "m = {} (Cấp lẻ): Lấy trực tiếp từ delta[{}][{}] = {:.4}",
                m, r, m, coeff[m]
            );
        }
    }

    // Bước 3: Chuẩn hóa hệ số theo cấp giai thừa và h^m
    let mut h_pow = 1.0;
    for m in 0..k {
        let fact = factorial(m);
        coeff[m] = coeff[m] / (fact * h_pow);
        h_pow *= h;
    }

    println!("\n=== [DEBUG] 3. HỆ SỐ ĐA THỨC CHUẨN HÓA (coeff theo t) ===");
    println!("=======================================================");
    for (m, &val) in coeff.iter().enumerate() {
        println!("a_{} = {:.6}", m, val);
    }
    println!("=======================================================\n");

    coeff
}
