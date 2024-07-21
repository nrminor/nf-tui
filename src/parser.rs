//! Module `parser` parses the `params {}` section of a Nextflow configuration file. The file's
//! name is either specified in `nftui.toml` or assumed to be `nextflow.config`. `parser` uses the
//! information in the params section and organizes it into an iterator of struct `Param`, which
//! bundles the parameter name together with any default (which is to say, non-null) values,
//! a description in a comment preceding the parameter, and an validation rule optionally
//! provided in the `nf-tui` TOML configuration file.
//!
//! Note that `nf-tui` will only force the user to input values for parameters that are `null`
//! in the config file, and that get parsed into the `None` variant of Rust's `Option` type.

use crate::rules::SupportedRules;

#[allow(dead_code)]
struct Param {
    pub name: String,
    pub default: Option<String>,
    pub description: Option<String>,
    pub rule: Option<SupportedRules>,
}
