
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
pub fn extract_central_points(points: &[(f64, f64)], x_bar: f64, k: usize) -> Option<Vec<(f64, f64)>> {
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
