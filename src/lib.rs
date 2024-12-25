//! CTE IR
//! Compile-time evaluation IR
//! 
//! This IR is designed to support compile-time evaluation.
//! All the types are values.
//! 


/// the builder module
pub mod builder;
/// the evalution module
pub mod evalute;
mod dump;
/// the core of CTE IR
pub mod ir;
