extern crate cbindgen;

use std::env;
use std::path::*;

fn main() {
  let out_dir =  env::var("OUT_DIR").unwrap();
  let out_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap().join("prust_main.h");
  let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let mut config: cbindgen::Config = Default::default();
  config.language = cbindgen::Language::C;
  cbindgen::generate_with_config(&crate_dir, config)
    .unwrap()
    .write_to_file(out_dir);
}