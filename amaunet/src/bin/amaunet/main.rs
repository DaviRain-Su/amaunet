//! Main entry point for Amaunet

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use amaunet::application::APP;

/// Boot Amaunet
fn main() {
    abscissa_core::boot(&APP);
}
