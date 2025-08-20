//! Field arithmetic for XFG STARK implementation

use super::{FieldElement, TypeError};

/// Prime field element for 64-bit operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimeField64 {
    value: u64,
}

impl PrimeField64 {
    /// Create a new field element
    pub fn new(value: u64) -> Self {
        Self { value }
    }
    
    /// Get the value
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl FieldElement for PrimeField64 {
    fn value(&self) -> u64 {
        self.value
    }
    
    fn new(value: u64) -> Self {
        Self::new(value)
    }
    
    fn zero() -> Self {
        Self { value: 0 }
    }
    
    fn one() -> Self {
        Self { value: 1 }
    }
    
    fn add(&self, other: &Self) -> Self {
        Self { value: self.value.wrapping_add(other.value) }
    }
    
    fn mul(&self, other: &Self) -> Self {
        Self { value: self.value.wrapping_mul(other.value) }
    }
    
    fn sub(&self, other: &Self) -> Self {
        Self { value: self.value.wrapping_sub(other.value) }
    }
    
    fn is_zero(&self) -> bool {
        self.value == 0
    }
    
    fn is_one(&self) -> bool {
        self.value == 1
    }
    
    fn from_bytes(bytes: &[u8; 32]) -> Option<Self> {
        // Simple conversion - take first 8 bytes
        let mut value = 0u64;
        for (i, &byte) in bytes.iter().take(8).enumerate() {
            value |= (byte as u64) << (i * 8);
        }
        Some(Self { value })
    }
    
    fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        for i in 0..8 {
            bytes[i] = ((self.value >> (i * 8)) & 0xFF) as u8;
        }
        bytes
    }
}

// Standard arithmetic trait implementations
impl std::ops::Add for PrimeField64 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Self { value: self.value.wrapping_add(other.value) }
    }
}

impl std::ops::AddAssign for PrimeField64 {
    fn add_assign(&mut self, other: Self) {
        self.value = self.value.wrapping_add(other.value);
    }
}

impl std::ops::Sub for PrimeField64 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Self { value: self.value.wrapping_sub(other.value) }
    }
}

impl std::ops::SubAssign for PrimeField64 {
    fn sub_assign(&mut self, other: Self) {
        self.value = self.value.wrapping_sub(other.value);
    }
}

impl std::ops::Mul for PrimeField64 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self::Output {
        Self { value: self.value.wrapping_mul(other.value) }
    }
}

impl std::ops::MulAssign for PrimeField64 {
    fn mul_assign(&mut self, other: Self) {
        self.value = self.value.wrapping_mul(other.value);
    }
}

impl std::ops::Neg for PrimeField64 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self { value: self.value.wrapping_neg() }
    }
}

impl std::fmt::Display for PrimeField64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
