//! Type definitions for XFG STARK implementation

pub mod field;
pub mod polynomial;
pub mod stark;
pub mod secret;

pub use field::*;
pub use polynomial::*;
pub use stark::*;
pub use secret::*;

/// Type error for the type system
#[derive(Debug, thiserror::Error)]
pub enum TypeError {
    #[error("Invalid field element")]
    InvalidFieldElement,
    #[error("Invalid polynomial")]
    InvalidPolynomial,
    #[error("Invalid STARK proof")]
    InvalidStarkProof,
    #[error("Invalid secret")]
    InvalidSecret,
}

/// Field element trait for cryptographic operations
pub trait FieldElement: Clone + Copy + PartialEq + Eq {
    /// Get the value as u64
    fn value(&self) -> u64;
    
    /// Create from u64
    fn new(value: u64) -> Self;
    
    /// Zero element
    fn zero() -> Self;
    
    /// One element
    fn one() -> Self;
    
    /// Add two elements
    fn add(&self, other: &Self) -> Self;
    
    /// Multiply two elements
    fn mul(&self, other: &Self) -> Self;
    
    /// Subtract two elements
    fn sub(&self, other: &Self) -> Self;
    
    /// Check if zero
    fn is_zero(&self) -> bool;
    
    /// Check if one
    fn is_one(&self) -> bool;
    
    /// Convert from bytes
    fn from_bytes(bytes: &[u8; 32]) -> Option<Self>;
    
    /// Convert to bytes
    fn to_bytes(&self) -> [u8; 32];
}

/// Trait for polynomial operations
pub trait Polynomial<F: FieldElement>: 
    Clone + std::fmt::Debug + std::fmt::Display + PartialEq + Eq
{
    /// Degree of the polynomial
    fn degree(&self) -> usize;
    
    /// Evaluate the polynomial at a point
    fn evaluate(&self, point: F) -> F;
    
    /// Get coefficient at given index
    fn coefficient(&self, index: usize) -> F;
    
    /// Set coefficient at given index
    fn set_coefficient(&mut self, index: usize, value: F);
    
    /// Add another polynomial
    fn add(&self, other: &Self) -> Self;
    
    /// Multiply by another polynomial
    fn multiply(&self, other: &Self) -> Self;
    
    /// Divide by another polynomial
    fn divide(&self, other: &Self) -> Option<(Self, Self)>;
    
    /// Compute the derivative
    fn derivative(&self) -> Self;
    
    /// Interpolate polynomial from points
    fn interpolate(points: &[(F, F)]) -> Option<Self>;
}

/// Trait for STARK proof components
pub trait StarkComponent<F: FieldElement>: 
    Clone + std::fmt::Debug + std::fmt::Display + PartialEq + Eq
{
    /// Validate the component
    fn validate(&self) -> Result<(), TypeError>;
    
    /// Serialize to bytes
    fn to_bytes(&self) -> Vec<u8>;
    
    /// Deserialize from bytes
    fn from_bytes(bytes: &[u8]) -> Result<Self, TypeError>;
}

/// Trait for secret types with secure zeroization
pub trait Secret: 
    Clone + std::fmt::Debug + PartialEq + Eq
{
    /// Zeroize the secret in memory
    fn zeroize(&mut self);
    
    /// Check if the secret is zeroized
    fn is_zeroized(&self) -> bool;
    
    /// Convert to bytes (constant-time)
    fn to_bytes(&self) -> Vec<u8>;
    
    /// Convert from bytes (constant-time)
    fn from_bytes(bytes: &[u8]) -> Result<Self, TypeError>;
}

/// Type-safe wrapper for cryptographic operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoType<T> {
    /// The underlying value
    value: T,
    /// Type safety marker
    _phantom: core::marker::PhantomData<T>,
}

impl<T> CryptoType<T> {
    /// Create a new cryptographic type
    pub fn new(value: T) -> Self {
        Self {
            value,
            _phantom: core::marker::PhantomData,
        }
    }
    
    /// Get the underlying value
    pub fn value(&self) -> &T {
        &self.value
    }
    
    /// Get mutable access to the underlying value
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    /// Consume and return the underlying value
    pub fn into_value(self) -> T {
        self.value
    }
}

/// Type-safe wrapper for constant-time operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstantTime<T> {
    /// The underlying value
    value: T,
    /// Constant-time marker
    _phantom: core::marker::PhantomData<T>,
}

impl<T> ConstantTime<T> {
    /// Create a new constant-time type
    pub fn new(value: T) -> Self {
        Self {
            value,
            _phantom: core::marker::PhantomData,
        }
    }
    
    /// Get the underlying value
    pub fn value(&self) -> &T {
        &self.value
    }
    
    /// Get mutable access to the underlying value
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    /// Consume and return the underlying value
    pub fn into_value(self) -> T {
        self.value
    }
}

/// Type-safe wrapper for memory-safe operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySafe<T> {
    /// The underlying value
    value: T,
    /// Memory safety marker
    _phantom: core::marker::PhantomData<T>,
}

impl<T> MemorySafe<T> {
    /// Create a new memory-safe type
    pub fn new(value: T) -> Self {
        Self {
            value,
            _phantom: core::marker::PhantomData,
        }
    }
    
    /// Get the underlying value
    pub fn value(&self) -> &T {
        &self.value
    }
    
    /// Get mutable access to the underlying value
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    /// Consume and return the underlying value
    pub fn into_value(self) -> T {
        self.value
    }
}

/// Type-safe result for cryptographic operations
pub type CryptoResult<T> = Result<T, TypeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_type() {
        let value = 42u64;
        let crypto_type = CryptoType::new(value);
        assert_eq!(*crypto_type.value(), value);
    }

    #[test]
    fn test_constant_time() {
        let value = 42u64;
        let ct_type = ConstantTime::new(value);
        assert_eq!(*ct_type.value(), value);
    }

    #[test]
    fn test_memory_safe() {
        let value = 42u64;
        let ms_type = MemorySafe::new(value);
        assert_eq!(*ms_type.value(), value);
    }
}
