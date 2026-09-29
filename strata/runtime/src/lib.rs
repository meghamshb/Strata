//! Paper-only process composition, lifecycle, and operator controls for Strata.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(not(feature = "paper-only"))]
compile_error!("Strata V0 permits paper and shadow execution only");
