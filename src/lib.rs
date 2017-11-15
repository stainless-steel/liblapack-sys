//! Bindings to the [Linear Algebra PACKage][lapack].
//!
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
#![allow(non_camel_case_types)]
#![no_std]

extern crate libc;

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src as raw;

/// A complex number with 64-bit parts.
#[allow(bad_style)]
pub type c_double_complex = [libc::c_double; 2];

/// A complex number with 32-bit parts.
#[allow(bad_style)]
pub type c_float_complex = [libc::c_float; 2];

#[cfg(not(feature = "accelerate"))]
pub mod c;

pub mod fortran;
