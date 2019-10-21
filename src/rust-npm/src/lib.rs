mod utils;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use wasm_bindgen::prelude::*;
use ndarray::Array2;



#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) -> std::string::String {
    let mut hello = "Hello ".to_owned();
    hello.push_str(name);
    return hello;
}


#[wasm_bindgen]
pub fn multiply(json_matrix_one: &str,json_matrix_two: &str) -> std::string::String {
    let matrix_one: Array2<f64> = serde_json::from_str(&json_matrix_one).unwrap();
    let matrix_two: Array2<f64>  = serde_json::from_str(&json_matrix_two).unwrap();

    let result = matrix_one.dot(&matrix_two);
    let result_string = serde_json::to_string(&result).unwrap();

    return result_string;
}

