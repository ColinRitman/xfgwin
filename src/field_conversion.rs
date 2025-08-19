//! Field Element Conversion Layer for XFG STARK Implementation
//! 
//! This module provides seamless conversion between xfg_stark PrimeField64
//! and Winterfell BaseElement, ensuring type safety and mathematical correctness.

use crate::types::field::PrimeField64;
use winterfell::math::fields::f64::BaseElement;

/// Field conversion trait for bridging xfg_stark and Winterfell field elements
pub trait FieldConverter {
    /// Convert xfg_stark PrimeField64 to Winterfell BaseElement
    fn xfg_to_winterfell(xfg_element: PrimeField64) -> BaseElement;
    
    /// Convert Winterfell BaseElement to xfg_stark PrimeField64
    fn winterfell_to_xfg(winterfell_element: BaseElement) -> PrimeField64;
}

impl FieldConverter for PrimeField64 {
    fn xfg_to_winterfell(xfg_element: PrimeField64) -> BaseElement {
        // Convert PrimeField64 to Winterfell BaseElement
        // Both use the same underlying field (F64), so we can safely convert
        BaseElement::from(xfg_element.value())
    }
    
    fn winterfell_to_xfg(winterfell_element: BaseElement) -> PrimeField64 {
        // Convert Winterfell BaseElement to PrimeField64
        PrimeField64::new(winterfell_element.as_int())
    }
}

/// Batch conversion utilities for field element arrays
pub trait BatchFieldConverter {
    /// Convert vector of xfg_stark field elements to Winterfell
    fn batch_xfg_to_winterfell(xfg_elements: &[PrimeField64]) -> Vec<BaseElement>;
    
    /// Convert vector of Winterfell field elements to xfg_stark
    fn batch_winterfell_to_xfg(winterfell_elements: &[BaseElement]) -> Vec<PrimeField64>;
}

impl BatchFieldConverter for PrimeField64 {
    fn batch_xfg_to_winterfell(xfg_elements: &[PrimeField64]) -> Vec<BaseElement> {
        xfg_elements.iter()
            .map(|&element| FieldConverter::xfg_to_winterfell(element))
            .collect()
    }
    
    fn batch_winterfell_to_xfg(winterfell_elements: &[BaseElement]) -> Vec<PrimeField64> {
        winterfell_elements.iter()
            .map(|&element| FieldConverter::winterfell_to_xfg(element))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_field_conversion_roundtrip() {
        let original = PrimeField64::new(12345);
        let converted = FieldConverter::xfg_to_winterfell(original);
        let back = FieldConverter::winterfell_to_xfg(converted);
        
        assert_eq!(original, back);
    }
    
    #[test]
    fn test_batch_field_conversion() {
        let xfg_elements = vec![
            PrimeField64::new(1),
            PrimeField64::new(2),
            PrimeField64::new(3),
        ];
        
        let winterfell_elements = BatchFieldConverter::batch_xfg_to_winterfell(&xfg_elements);
        let back_to_xfg = BatchFieldConverter::batch_winterfell_to_xfg(&winterfell_elements);
        
        assert_eq!(xfg_elements, back_to_xfg);
    }
    
    #[test]
    fn test_zero_conversion() {
        let xfg_zero = PrimeField64::zero();
        let winterfell_zero = FieldConverter::xfg_to_winterfell(xfg_zero);
        
        assert_eq!(winterfell_zero, BaseElement::ZERO);
    }
    
    #[test]
    fn test_one_conversion() {
        let xfg_one = PrimeField64::one();
        let winterfell_one = FieldConverter::xfg_to_winterfell(xfg_one);
        
        assert_eq!(winterfell_one, BaseElement::ONE);
    }
}