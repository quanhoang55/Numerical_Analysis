// =============================================================
// PURPOSE: ERROR HANDLING
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use thiserror::Error;

// =============================================================
// CLASSES / DATA STRUCTURE: InterpolationError
// =============================================================
#[derive(Debug, Error, PartialEq)]
pub enum InterpolationError {
    #[error("Cần tối thiểu {required} điểm nút, nhưng chỉ nhận được {provided}")]
    InsufficientPoints { required: usize, provided: usize },

    #[error("Các điểm nút x bị trùng lặp tại x = {0}")]
    DuplicateNode(String),

    #[error("Mảng x không được sắp xếp tăng dần")]
    UnsortedNodes,

    #[error("Không thể giải hệ phương trình tuyến tính")]
    LinearSolverFailure,
}
