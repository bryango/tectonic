// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::env;

fn main() {
    // Re-export $TARGET during the build so that our executable tests know
    // what environment variable CARGO_TARGET_@TARGET@_RUNNER to check when
    // they want to spawn off executables.
    let target = env::var("TARGET").unwrap();
    println!("cargo:rustc-env=TARGET={target}");

    // Set the default web bundle for tectonic to use.
    // The `${TECTONIC_WEB_BUNDLE_PREFIX}` would lead to a url in the form of
    // `${TECTONIC_WEB_BUNDLE_PREFIX}/default_bundle.tar`, while the "locked"
    // url, if specified, can be used to pin the default bundle to a specific
    // version. This would be useful for reproducible builds. Empty variables
    // lead to a hardcoded fallback, currently defined in `crates/bundles/src/lib.rs`.
    let web_bundle_prefix = env::var("TECTONIC_WEB_BUNDLE_PREFIX").unwrap_or("".to_owned());
    let web_bundle_locked = env::var("TECTONIC_WEB_BUNDLE_LOCKED").unwrap_or("".to_owned());
    println!("cargo:rustc-env=TECTONIC_WEB_BUNDLE_PREFIX={web_bundle_prefix}");
    println!("cargo:rustc-env=TECTONIC_WEB_BUNDLE_LOCKED={web_bundle_locked}");
}
