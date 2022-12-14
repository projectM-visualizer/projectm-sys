use std::{path::{Path, PathBuf}, env};

pub fn bindgen() {
  println!("cargo:rerun-if-changed=wrapper.h");
  
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let out_path = Path::new(".").join("src");
  let bindings = bindgen::Builder::default()
      .header("wrapper.h")
      .allowlist_function("projectm_.*")
      .clang_arg(&format!("-I{}/include/libprojectM", out_dir.display()))
      .parse_callbacks(Box::new(bindgen::CargoCallbacks))
      .generate()
      .expect("Unable to generate bindings");

  bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");
}