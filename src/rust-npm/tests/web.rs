//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
    assert_eq!(rust_npm::greet("Dan"),"Hello Dan");
    assert_eq!(rust_npm::multiply(&"{ \"v\": 1, \"data\":[1.3,2.3,1.2,3.2], \"dim\":[2,2]}",&"{ \"v\": 1,\"data\":[1.1,1.3,5.6,2.4], \"dim\":[2,2]}"),"{\"v\":1,\"dim\":[2,2],\"data\":[14.309999999999999,7.21,19.24,9.24]}");
}
