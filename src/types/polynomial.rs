//! Polynomial operations for XFG STARK implementation

use super::{FieldElement, TypeError};

/// Polynomial error
#[derive(Debug, thiserror::Error)]
pub enum PolynomialError {
    #[error("Invalid polynomial")]
    InvalidPolynomial,
}

/// Polynomial over a field
#[derive(Debug, Clone)]
pub struct Polynomial<F: FieldElement> {
    coefficients: Vec<F>,
}

impl<F: FieldElement> Polynomial<F> {
    /// Create a new polynomial
    pub fn new(coefficients: Vec<F>) -> Self {
        Self { coefficients }
    }
    
    /// Get the degree
    pub fn degree(&self) -> usize {
        if self.coefficients.is_empty() {
            0
        } else {
            self.coefficients.len() - 1
        }
    }
    
    /// Evaluate at a point
    pub fn evaluate(&self, point: &F) -> F {
        let mut result = F::zero();
        let mut power = F::one();
        
        for &coeff in &self.coefficients {
            result = result.add(&coeff.mul(&power));
            power = power.mul(point);
        }
        
        result
    }
}
