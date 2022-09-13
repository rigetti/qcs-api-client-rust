//! Implementation code common to the QCS OpenAPI and gRPC clients.
//!
//! You probably don't need to use this directly, as the clients should expose anything you might
//! need.

// Covers correctness, suspicious, style, complexity, and perf
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![warn(clippy::nursery)]
// Has false positives that conflict with unreachable_pub
#![allow(clippy::redundant_pub_crate)]
#![deny(rustdoc::missing_doc_code_examples)]
#![deny(
absolute_paths_not_starting_with_crate,
anonymous_parameters,
bad_style,
const_err,
dead_code,
keyword_idents,
improper_ctypes,
macro_use_extern_crate,
meta_variable_misuse, // May have false positives
missing_abi,
missing_debug_implementations, // can affect compile time/code size
missing_docs,
no_mangle_generic_items,
non_shorthand_field_patterns,
noop_method_call,
overflowing_literals,
path_statements,
patterns_in_fns_without_body,
pointer_structural_match,
private_in_public,
semicolon_in_expressions_from_macros,
trivial_casts,
trivial_numeric_casts,
unaligned_references,
unconditional_recursion,
unreachable_pub,
unsafe_code,
unused,
unused_allocation,
unused_comparisons,
unused_extern_crates,
unused_import_braces,
unused_lifetimes,
unused_parens,
unused_qualifications,
variant_size_differences,
while_true
)]

pub mod configuration;
pub use configuration::ClientConfiguration;

#[cfg(feature = "grpc")]
pub mod grpc;
