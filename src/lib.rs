extern crate cfg_if;
extern crate wasm_bindgen;
extern crate serde_json;


mod utils;

use serde_json::{Value, Error};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
  #[wasm_bindgen(js_namespace = console)]
  fn log(msg: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
  ($($t:tt)*) => (log(&format!($($t)*)))
}

fn untyped_example(json: &str) -> Result<Value, Error> {

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(json)?;

    // Access parts of the data by indexing with square brackets.
    log!("json name {}", v["name"]);

    Ok(v["name"].take())
}

#[wasm_bindgen]
pub fn controllSumJSON(json: &str) -> String {
  match untyped_example(json) {
    Ok(result) => "1".to_string(),
    Err(err) => "0".to_string(),
  }
}
