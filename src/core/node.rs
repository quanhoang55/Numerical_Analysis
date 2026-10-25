// =============================================================
// PURPOSE: CHEBYSHEV NODE
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use num_traits::Float;
use std::f64::consts::PI;

// =============================================================
// CLASSES / DATA STRUCTURE: ChebyshevNode
// =============================================================
#[derive(Debug)]
pub struct ChebyshevNode;

impl ChebyshevNode {
    pub fn find_nodes<T: Float>(a: T, b: T, n: usize) -> Option<Vec<T>> {
        if a >= b {
            return None;
        }
        let mut result: Vec<T> = Vec::with_capacity(n + 1);

        let two = T::from(2.0).unwrap();
        let pi = T::from(PI).unwrap();
        let n_t = T::from(n).unwrap();

        let center = (a + b) / two;
        let half = (b - a) / two;
        for i in (0..=n).rev() {
            let k = T::from(i).unwrap();
            let theta = pi * (two * k + T::one()) / (two * n_t + two);
            result.push(center + half * theta.cos());
        }
        Some(result)
    }
}

// =============================================================
// TESTS
// =============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chebyshev_nodes_count_and_order() {
        let a = 0.0f64;
        let b = 4.0f64;
        let n = 2;

        let nodes = ChebyshevNode::find_nodes(a, b, n).expect("Error: Can't find Node");

        assert_eq!(nodes.len(), n + 1);

        for window in nodes.windows(2) {
            assert!(
                window[0] < window[1],
                "Nodes: {} >= {}",
                window[0],
                window[1]
            );
        }

        let expected_x0 = 2.0 - 3.0f64.sqrt(); // ~0.2679
        let expected_x1 = 2.0; // 2.0
        let expected_x2 = 2.0 + 3.0f64.sqrt(); // ~3.7320

        let epsilon = 1e-6;
        assert!((nodes[0] - expected_x0).abs() < epsilon);
        assert!((nodes[1] - expected_x1).abs() < epsilon);
        assert!((nodes[2] - expected_x2).abs() < epsilon);
    }

    #[test]
    fn test_invalid_interval() {
        let result = ChebyshevNode::find_nodes(5.0f64, 2.0f64, 3);
        assert!(result.is_none());
    }
}
