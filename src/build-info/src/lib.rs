// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

// BEGIN LINT CONFIG
// DO NOT EDIT. Automatically generated by bin/gen-lints.
// Have complaints about the noise? See the note in misc/python/materialize/cli/gen-lints.py first.
#![allow(clippy::style)]
#![allow(clippy::complexity)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::mutable_key_type)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::map_entry)]
#![allow(clippy::box_default)]
#![warn(clippy::bool_comparison)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::no_effect)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::zero_prefixed_literal)]
#![warn(clippy::borrowed_box)]
#![warn(clippy::deref_addrof)]
#![warn(clippy::double_must_use)]
#![warn(clippy::double_parens)]
#![warn(clippy::extra_unused_lifetimes)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_question_mark)]
#![warn(clippy::needless_return)]
#![warn(clippy::redundant_pattern)]
#![warn(clippy::redundant_slicing)]
#![warn(clippy::redundant_static_lifetimes)]
#![warn(clippy::single_component_path_imports)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::useless_asref)]
#![warn(clippy::useless_conversion)]
#![warn(clippy::builtin_type_shadow)]
#![warn(clippy::duplicate_underscore_argument)]
#![warn(clippy::double_neg)]
#![warn(clippy::unnecessary_mut_passed)]
#![warn(clippy::wildcard_in_or_patterns)]
#![warn(clippy::collapsible_if)]
#![warn(clippy::collapsible_else_if)]
#![warn(clippy::crosspointer_transmute)]
#![warn(clippy::excessive_precision)]
#![warn(clippy::overflow_check_conditional)]
#![warn(clippy::as_conversions)]
#![warn(clippy::match_overlapping_arm)]
#![warn(clippy::zero_divided_by_zero)]
#![warn(clippy::must_use_unit)]
#![warn(clippy::suspicious_assignment_formatting)]
#![warn(clippy::suspicious_else_formatting)]
#![warn(clippy::suspicious_unary_op_formatting)]
#![warn(clippy::mut_mutex_lock)]
#![warn(clippy::print_literal)]
#![warn(clippy::same_item_push)]
#![warn(clippy::useless_format)]
#![warn(clippy::write_literal)]
#![warn(clippy::redundant_closure)]
#![warn(clippy::redundant_closure_call)]
#![warn(clippy::unnecessary_lazy_evaluations)]
#![warn(clippy::partialeq_ne_impl)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::transmutes_expressible_as_ptr_casts)]
#![warn(clippy::unused_async)]
#![warn(clippy::disallowed_methods)]
#![warn(clippy::disallowed_macros)]
#![warn(clippy::disallowed_types)]
#![warn(clippy::from_over_into)]
// END LINT CONFIG

//! Metadata about a Materialize build.
//!
//! These types are located in a dependency-free crate so they can be used
//! from any layer of the stack.

/// Build information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildInfo {
    /// The version number of the build.
    pub version: &'static str,
    /// The 40-character SHA-1 hash identifying the Git commit of the build.
    pub sha: &'static str,
    /// The time of the build in UTC as an ISO 8601-compliant string.
    pub time: &'static str,
}

/// Dummy build information.
///
/// Intended for use in contexts where getting the correct build information is
/// impossible or unnecessary, like in tests.
pub const DUMMY_BUILD_INFO: BuildInfo = BuildInfo {
    version: "0.0.0+dummy",
    sha: "0000000000000000000000000000000000000000",
    time: "",
};

/// The target triple of the platform.
pub const TARGET_TRIPLE: &str = env!("TARGET_TRIPLE");

impl BuildInfo {
    /// Constructs a human-readable version string.
    pub fn human_version(&self) -> String {
        format!("v{} ({})", self.version, &self.sha[..9])
    }

    /// Returns the version as a rich [semantic version][semver].
    ///
    /// This method is only available when the `semver` feature is active.
    ///
    /// # Panics
    ///
    /// Panics if the `version` field is not a valid semantic version.
    ///
    /// [semver]: https://semver.org
    #[cfg(feature = "semver")]
    pub fn semver_version(&self) -> semver::Version {
        self.version
            .parse()
            .expect("build version is not valid semver")
    }

    /// Returns the version as an integer along the lines of Pg's server_version_num
    #[cfg(feature = "semver")]
    pub fn version_num(&self) -> i32 {
        let semver: semver::Version = self
            .version
            .parse()
            .expect("build version is not a valid semver");
        let ver_string = format!(
            "{:0>2}{:0>3}{:0>2}",
            semver.major, semver.minor, semver.patch
        );
        ver_string.parse::<i32>().unwrap()
    }
}

/// Generates an appropriate [`BuildInfo`] instance.
///
/// This macro should be invoked at the leaf of the crate graph, usually in the
/// final binary, and the resulting `BuildInfo` struct plumbed into whatever
/// libraries require it. Invoking the macro in intermediate crates may result
/// in a build info with stale, cached values for the build SHA and time.
#[macro_export]
macro_rules! build_info {
    () => {
        $crate::BuildInfo {
            version: env!("CARGO_PKG_VERSION"),
            sha: $crate::private::run_command_str!(
                "sh",
                "-c",
                r#"if [ -n "$MZ_DEV_BUILD_SHA" ]; then
                       echo "$MZ_DEV_BUILD_SHA"
                   else
                       # Unfortunately we need to suppress error messages from `git`, as
                       # run_command_str will display no error message at all if we print
                       # more than one line of output to stderr.
                       git rev-parse --verify HEAD 2>/dev/null || {
                           printf "error: unable to determine Git SHA; " >&2
                           printf "either build from working Git clone " >&2
                           printf "(see https://materialize.com/docs/install/#build-from-source), " >&2
                           printf "or specify SHA manually by setting MZ_DEV_BUILD_SHA environment variable" >&2
                           exit 1
                       }
                   fi"#
            ),
            time: $crate::private::run_command_str!("date", "-u", "+%Y-%m-%dT%H:%M:%SZ"),
        }
    }
}

#[doc(hidden)]
pub mod private {
    pub use compile_time_run::run_command_str;
}
