// This tests that `const_trait` default methods can
// be called from a const context when used across crates.
//
//@ check-pass

#![feature(const_trait_impl, effects)] //~ WARN the feature `effects` is incomplete

//@ aux-build: cross-crate.rs
extern crate cross_crate;

use cross_crate::*;

const _: () = {
    Const.func();
    Const.defaulted_func();
};

fn main() {}
