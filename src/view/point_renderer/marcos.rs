#![macro_use]
/// Some quick and dirty marcros for verifying the output exit codes of GL ffi calls

#[allow(unused_imports)]
use kiss3d::context::Context;

#[allow(unused_macros)]
macro_rules! verify(
    ($e: expr) => {
        {
            let res = $e;
            { assert_eq!(Context::get().get_error(), 0); }
            res
        }
    }
);

#[allow(unused_macros)]
macro_rules! ignore(
    ($e: expr) => {
        {
            let res = $e;
            { let _ = Context::get().get_error(); }
            res
        }
    }
);

#[allow(unused_macros)]
macro_rules! checked(
    ($e: expr) => {
        {
            let res = $e;
            match Context::get().get_error() {
                0 => Some(res)
                _ => None,
            }
        }
    }
);
