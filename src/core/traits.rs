// =============================================================
// PURPOSE: POINT
// =============================================================
// IMPORTS & MODULE LOADING
// =============================================================
use num_traits::Float;

// =============================================================
// CLASSES / DATA STRUCTURE: Point
// =============================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn dx(&self, other: &Self) -> T {
        other.x - self.x
    }
    pub fn dx_abs(&self, other: &Self) -> T {
        (other.x - self.x).abs()
    }

    pub fn dy(&self, other: &Self) -> T {
        other.y - self.y
    }
    pub fn dy_abs(&self, other: &Self) -> T {
        (other.y - self.y).abs()
    }

    pub fn slop(&self, other: &Self) -> T {
        self.dy(other) / self.dx(other)
    }

    pub fn distance_to(&self, other: &Self) -> T {
        let dx = self.dx(other);
        let dy = self.dy(other);
        (dx * dx + dy * dy).sqrt()
    }
}
