
// =============================================================
// PURPOSE: HORNER
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use csv;
use calamine::{Reader, Xlsx, open_workbook};

// =============================================================
// CORE LOGIC & FUNCTIONS
// =============================================================


/// 1. Đọc dữ liệu điểm (x, y) từ file văn bản (.txt)
pub fn read_txt_points<P: AsRef<Path>>(path: P) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .collect();

        if parts.len() >= 2 {
            let x: f64 = parts[0].parse()?;
            let y: f64 = parts[1].parse()?;
            points.push((x, y));
        }
    }

    Ok(points)
}

/// 2. Đọc dữ liệu điểm (x, y) từ file CSV (.csv)
pub fn read_csv_points<P: AsRef<Path>>(path: P) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut points = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.len() >= 2 {
            let x: f64 = record[0].parse()?;
            let y: f64 = record[1].parse()?;
            points.push((x, y));
        }
    }

    Ok(points)
}

/// 3. Đọc dữ liệu điểm (x, y) từ file Excel (.xlsx)
pub fn read_xlsx_points<P: AsRef<Path>>(path: P) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(path)?;
    let sheet_name = excel
        .sheet_names()
        .first()
        .ok_or("Không tìm thấy sheet nào trong file Excel")?
        .clone();

    let range = excel
        .worksheet_range(&sheet_name)
        .map_err(|_| "Không thể đọc dữ liệu từ sheet")?;

    let mut points = Vec::new();

    for row in range.rows() {
        if row.len() >= 2 {
            // Dùng trait Display của calamine cell để format sang string rồi parse an toàn sang f64
            let parse_cell = |cell| -> Option<f64> {
                let s = format!("{}", cell);
                s.trim().parse::<f64>().ok()
            };

            if let (Some(x), Some(y)) = (parse_cell(&row[0]), parse_cell(&row[1])) {
                points.push((x, y));
            }
        }
    }

    Ok(points)
}
