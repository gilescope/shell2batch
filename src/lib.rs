#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    array_into_iter,
    bindings_with_variant_name,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incomplete_features,
    indirect_structural_match,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_patterns,
    path_statements,
    patterns_in_fns_without_body,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolon,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    intra_doc_link_resolution_failure,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # shell2batch
//!
//! Coverts simple basic shell scripts to windows batch scripts.
//!
//! While it is not really possible to take every shell script and automatically convert it to a windows batch file,
//! this library provides a way to convert simple basic shell commands to windows batch commands.<br>
//! The original goal of this library is to provide users of [cargo-make](https://sagiegurari.github.io/cargo-make/) a
//! way to write simple tasks with shell scripts without duplicating their code for each platform.
//!
//! It is possible to provide custom conversion hints by using the ```# shell2batch:``` prefix (see below example).
//!
//! # Examples
//!
//! ```
//! extern crate shell2batch;
//!
//! fn main() {
//!     let script = shell2batch::convert(
//!         r#"
//!         export FILE1=file1
//!         export FILE2=file2
//!
//!         #this is some test code
//!         cp ${FILE1} $FILE2
//!         cp -r ${DIR1} $DIR2
//!
//!         #another
//!         mv file2 file3
//!
//!         export MY_DIR=directory
//!
//!         #flags are supported
//!         rm -Rf ${MY_DIR}
//!
//!         unset MY_DIR
//!
//!         #provide custom windows command for specific shell command
//!         complex_bash_command --flag1 value2 # shell2batch: complex_windows_command /flag10 windows_value
//!         "#
//!     );
//!
//!     assert_eq!(
//!         script,
//!         r#"
//! set FILE1=file1
//! set FILE2=file2
//!
//! @REM this is some test code
//! copy %FILE1% %FILE2%
//! xcopy /E %DIR1% %DIR2%
//!
//! @REM another
//! move file2 file3
//!
//! set MY_DIR=directory
//!
//! @REM flags are supported
//! rmdir /S /Q %MY_DIR%
//!
//! set MY_DIR=
//!
//! @REM provide custom windows command for specific shell command
//! complex_windows_command /flag10 windows_value
//! "#
//!     );
//!
//!     println!("Script: {}", script);
//! }
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/shell2batch/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/shell2batch/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

mod converter;

/// Converts the provided shell script and returns the windows batch script text.
///
/// # Example
///
/// ```
/// extern crate shell2batch;
///
/// fn main() {
///     let script = shell2batch::convert(
///         r#"
///         set -x
///
///         export FILE1=file1
///         export FILE2=file2
///
///         #this is some test code
///         cp ${FILE1} $FILE2
///         cp -r ${DIR1} $DIR2
///
///         #another
///         mv file2 file3
///
///         export MY_DIR=directory
///
///         #flags are supported
///         rm -Rf ${MY_DIR}
///
///         unset MY_DIR
///
///         touch ./file3
///
///         #provide custom windows command for specific shell command
///         complex_bash_command --flag1 value2 # shell2batch: complex_windows_command /flag10 windows_value
///         "#
///     );
///
///     assert_eq!(
///         script,
///         r#"
/// @echo on
///
/// set FILE1=file1
/// set FILE2=file2
///
/// @REM this is some test code
/// copy %FILE1% %FILE2%
/// xcopy /E %DIR1% %DIR2%
///
/// @REM another
/// move file2 file3
///
/// set MY_DIR=directory
///
/// @REM flags are supported
/// rmdir /S /Q %MY_DIR%
///
/// set MY_DIR=
///
/// copy /B .\file3+,, .\file3
///
/// @REM provide custom windows command for specific shell command
/// complex_windows_command /flag10 windows_value
/// "#
///     );
///
///     println!("Script: {}", script);
/// }
/// ```
pub fn convert(script: &str) -> String {
    converter::run(script)
}
