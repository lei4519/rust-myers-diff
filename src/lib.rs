extern crate wasm_bindgen;

use myers::{myers};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

pub mod myers;

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub old_str: Vec<String>,
    pub new_str: Vec<String>,
}

#[wasm_bindgen]
pub fn diff(params: JsValue) -> JsValue {
	let params: Params = params.into_serde().unwrap();
	let res = myers(params.old_str, params.new_str);
	JsValue::from_serde(&res).unwrap()
}
