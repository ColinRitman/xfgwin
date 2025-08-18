//! Performance Benchmarks and Optimization
//! 
//! This module provides comprehensive benchmarking tools for measuring and optimizing
//! the performance of STARK proof generation and verification.
//! 
//! ## Features
//! 
//! - **Component Benchmarks**: Individual component performance testing
//! - **End-to-End Benchmarks**: Complete proof generation and verification timing
//! - **Memory Profiling**: Memory usage analysis and optimization
//! - **Scalability Testing**: Performance scaling with input size
//! - **Optimization Recommendations**: Automated performance suggestions

use crate::types::FieldElement;
use crate::types::field::PrimeField64;
use crate::proof::StarkProver;
use crate::proof::fri::FriProver;
use crate::proof::merkle::MerkleTree;
use crate::air::{Air, Constraint, TransitionFunction, BoundaryConditions};
use crate::air::constraints::ConstraintType;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Operation name
    pub operation: String,
    /// Execution time
    pub duration: Duration,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// Number of iterations
    pub iterations: usize,
    /// Input size
    pub input_size: usize,
    /// Additional metrics
    pub metrics: HashMap<String, f64>,
}

impl BenchmarkResult {
    /// Create a new benchmark result
    pub fn new(operation: String, duration: Duration, input_size: usize) -> Self {
        Self {
            operation,
            duration,
            memory_usage: 0,
            iterations: 1,
            input_size,
            metrics: HashMap::new(),
        }
    }

    /// Add a metric
    pub fn add_metric(&mut self, key: String, value: f64) {
        self.metrics.insert(key, value);
    }

    /// Get operations per second
    pub fn ops_per_second(&self) -> f64 {
        if self.duration.as_secs_f64() > 0.0 {
            self.iterations as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get throughput (operations per second)
    pub fn throughput(&self) -> f64 {
        self.ops_per_second()
    }

    /// Get average time per operation
    pub fn avg_time_per_op(&self) -> Duration {
        if self.iterations > 0 {
            Duration::from_nanos(self.duration.as_nanos() as u64 / self.iterations as u64)
        } else {
            Duration::ZERO
        }
    }
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} ops/sec, {} avg, {} bytes, {} iterations",
            self.operation,
            self.ops_per_second(),
            format_duration(self.avg_time_per_op()),
            self.memory_usage,
            self.iterations
        )
    }
}

/// Benchmark suite for STARK components
#[derive(Debug)]
pub struct BenchmarkSuite<F: FieldElement> {
    /// Field type
    _phantom: std::marker::PhantomData<F>,
    /// Results storage
    results: Vec<BenchmarkResult>,
}

impl<F: FieldElement> BenchmarkSuite<F> {
    /// Create a new benchmark suite
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            results: Vec::new(),
        }
    }

    /// Run field arithmetic benchmarks
    pub fn benchmark_field_arithmetic(&mut self, iterations: usize) {
        let start = Instant::now();
        
        for _ in 0..iterations {
            let a = F::random();
            let b = F::random();
            let _c = a + b;
            let _d = a * b;
            let _e = a.inverse();
        }
        
        let duration = start.elapsed();
        let result = BenchmarkResult::new(
            "Field Arithmetic".to_string(),
            duration,
            iterations,
        );
        
        self.results.push(result);
    }

    /// Run polynomial operation benchmarks
    pub fn benchmark_polynomial_operations(&mut self, degree: usize, iterations: usize) {
        let start = Instant::now();
        
        for _ in 0..iterations {
            let poly1 = generate_random_polynomial::<F>(degree);
            let poly2 = generate_random_polynomial::<F>(degree);
            // Simple polynomial operations (element-wise addition)
            let _sum: Vec<F> = poly1.iter().zip(poly2.iter()).map(|(a, b)| *a + *b).collect();
            // Simple polynomial operations (element-wise multiplication)
            let _product: Vec<F> = poly1.iter().zip(poly2.iter()).map(|(a, b)| *a * *b).collect();
        }
        
        let duration = start.elapsed();
        let mut result = BenchmarkResult::new(
            "Polynomial Operations".to_string(),
            duration,
            degree,
        );
        result.iterations = iterations;
        result.add_metric("degree".to_string(), degree as f64);
        
        self.results.push(result);
    }

    /// Run FRI proof generation benchmarks
    pub fn benchmark_fri_proof(&mut self, polynomial_size: usize, iterations: usize) {
        let prover = FriProver::<F>::new(128);
        let polynomial = generate_random_polynomial::<F>(polynomial_size);
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _proof = prover.prove(&polynomial);
        }
        
        let duration = start.elapsed();
        let mut result = BenchmarkResult::new(
            "FRI Proof Generation".to_string(),
            duration,
            polynomial_size,
        );
        result.iterations = iterations;
        result.add_metric("polynomial_size".to_string(), polynomial_size as f64);
        
        self.results.push(result);
    }

    /// Run Merkle tree benchmarks
    pub fn benchmark_merkle_tree(&mut self, num_leaves: usize, iterations: usize) {
        let leaves: Vec<Vec<u8>> = (0..num_leaves)
            .map(|i| format!("leaf_{}", i).into_bytes())
            .collect();
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let tree = MerkleTree::new(&leaves);
            if let Ok(tree) = tree {
                let _proof = tree.generate_proof(0);
            }
        }
        
        let duration = start.elapsed();
        let mut result = BenchmarkResult::new(
            "Merkle Tree Construction".to_string(),
            duration,
            num_leaves,
        );
        result.iterations = iterations;
        result.add_metric("num_leaves".to_string(), num_leaves as f64);
        
        self.results.push(result);
    }

    /// Run complete STARK proof benchmarks
    pub fn benchmark_stark_proof(&mut self, trace_size: usize, iterations: usize) {
        let prover = StarkProver::new(128);
        let air = create_test_air::<F>();
        let initial_state = vec![F::zero(); 2];
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _proof = prover.prove(&air, &initial_state, trace_size);
        }
        
        let duration = start.elapsed();
        let mut result = BenchmarkResult::new(
            "STARK Proof Generation".to_string(),
            duration,
            trace_size,
        );
        result.iterations = iterations;
        result.add_metric("trace_size".to_string(), trace_size as f64);
        
        self.results.push(result);
    }

    /// Run scalability benchmarks
    pub fn benchmark_scalability(&mut self, sizes: &[usize]) {
        for &size in sizes {
            self.benchmark_field_arithmetic(1000);
            self.benchmark_polynomial_operations(size, 100);
            self.benchmark_fri_proof(size, 10);
            self.benchmark_merkle_tree(size, 10);
            self.benchmark_stark_proof(size, 5);
        }
    }

    /// Get all results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Generate performance report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== STARK Performance Benchmark Report ===\n\n");
        
        for result in &self.results {
            report.push_str(&format!("{}\n", result));
        }
        
        report.push_str("\n=== Performance Analysis ===\n");
        report.push_str(&self.analyze_performance());
        
        report
    }

    /// Analyze performance and provide recommendations
    fn analyze_performance(&self) -> String {
        let mut analysis = String::new();
        
        // Find bottlenecks
        let mut slowest_ops = self.results.clone();
        slowest_ops.sort_by(|a, b| b.duration.cmp(&a.duration));
        
        analysis.push_str("Slowest operations:\n");
        for (i, result) in slowest_ops.iter().take(3).enumerate() {
            analysis.push_str(&format!("{}. {}: {}\n", 
                i + 1, 
                result.operation, 
                format_duration(result.duration)
            ));
        }
        
        analysis.push_str("\nOptimization recommendations:\n");
        
        // Generate recommendations based on results
        for result in &self.results {
            if result.ops_per_second() < 1000.0 {
                analysis.push_str(&format!("- {}: Consider optimization ({} ops/sec)\n", 
                    result.operation, result.ops_per_second()));
            }
        }
        
        analysis
    }
}

/// Performance profiler
#[derive(Debug)]
pub struct PerformanceProfiler {
    /// Profiling data
    measurements: HashMap<String, Vec<Duration>>,
}

impl PerformanceProfiler {
    /// Create a new profiler
    pub fn new() -> Self {
        Self {
            measurements: HashMap::new(),
        }
    }

    /// Start profiling a section
    pub fn start_section(&mut self, name: &str) -> ProfilerSection {
        ProfilerSection {
            name: name.to_string(),
            start: Instant::now(),
        }
    }

    /// Record a measurement
    pub fn record(&mut self, name: &str, duration: Duration) {
        self.measurements
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(duration);
    }

    /// Get profiling report
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Performance Profiling Report ===\n\n");
        
        for (name, measurements) in &self.measurements {
            let total: Duration = measurements.iter().sum();
            let avg = total / measurements.len() as u32;
            let min = measurements.iter().min().unwrap_or(&Duration::ZERO);
            let max = measurements.iter().max().unwrap_or(&Duration::ZERO);
            
            report.push_str(&format!("{}:\n", name));
            report.push_str(&format!("  Total: {}\n", format_duration(total)));
            report.push_str(&format!("  Average: {}\n", format_duration(avg)));
            report.push_str(&format!("  Min: {}\n", format_duration(*min)));
            report.push_str(&format!("  Max: {}\n", format_duration(*max)));
            report.push_str(&format!("  Count: {}\n\n", measurements.len()));
        }
        
        report
    }
}

/// Profiler section for RAII-style profiling
pub struct ProfilerSection {
    name: String,
    start: Instant,
}

impl ProfilerSection {
    /// End profiling and record the measurement
    pub fn end(self, profiler: &mut PerformanceProfiler) {
        let duration = self.start.elapsed();
        profiler.record(&self.name, duration);
    }
}

/// Memory usage tracker
#[derive(Debug)]
pub struct MemoryTracker {
    /// Memory measurements
    measurements: Vec<(String, usize)>,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }

    /// Track memory usage
    pub fn track(&mut self, operation: &str, usage: usize) {
        self.measurements.push((operation.to_string(), usage));
    }

    /// Get memory report
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Memory Usage Report ===\n\n");
        
        for (operation, usage) in &self.measurements {
            report.push_str(&format!("{}: {} bytes ({:.2} MB)\n", 
                operation, usage, *usage as f64 / 1024.0 / 1024.0));
        }
        
        if let Some(max_usage) = self.measurements.iter().map(|(_, usage)| usage).max() {
            report.push_str(&format!("\nPeak memory usage: {} bytes ({:.2} MB)\n", 
                max_usage, *max_usage as f64 / 1024.0 / 1024.0));
        }
        
        report
    }
}

/// Utility functions

/// Generate random polynomial
fn generate_random_polynomial<F: FieldElement>(degree: usize) -> Vec<F> {
    (0..degree).map(|_| F::random()).collect()
}

/// Create test AIR
fn create_test_air<F: FieldElement>() -> Air<F> {
    let constraints = vec![
        Constraint::new(vec![F::one(), F::zero()], 1, ConstraintType::Transition),
    ];
    
    let transition = TransitionFunction::new(vec![vec![F::one()]], 1);
    let boundary = BoundaryConditions::new(vec![]);
    
    Air::new(constraints, transition, boundary, 128)
}

/// Format duration for display
fn format_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.3}s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_micros() > 0 {
        format!("{}μs", duration.as_micros())
    } else {
        format!("{}ns", duration.as_nanos())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::field::PrimeField64;

    #[test]
    fn test_benchmark_suite_creation() {
        let suite = BenchmarkSuite::<PrimeField64>::new();
        assert_eq!(suite.results().len(), 0);
    }

    #[test]
    fn test_field_arithmetic_benchmark() {
        let mut suite = BenchmarkSuite::<PrimeField64>::new();
        suite.benchmark_field_arithmetic(100);
        assert_eq!(suite.results().len(), 1);
    }

    #[test]
    fn test_polynomial_benchmark() {
        let mut suite = BenchmarkSuite::<PrimeField64>::new();
        suite.benchmark_polynomial_operations(10, 10);
        assert_eq!(suite.results().len(), 1);
    }

    #[test]
    fn test_profiler() {
        let mut profiler = PerformanceProfiler::new();
        {
            let section = profiler.start_section("test");
            std::thread::sleep(Duration::from_millis(1));
            section.end(&mut profiler);
        }
        
        let report = profiler.report();
        assert!(report.contains("test"));
    }

    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        tracker.track("test", 1024);
        
        let report = tracker.report();
        assert!(report.contains("test"));
        assert!(report.contains("1024"));
    }
}