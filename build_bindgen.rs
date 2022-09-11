use std::{path::{Path, PathBuf}, env};

pub fn bindgen() {
  // Tell cargo to invalidate the built crate whenever the wrapper changes
  println!("cargo:rerun-if-changed=wrapper.h");
  
  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let out_path = Path::new(".").join("src");
  let bindings = bindgen::Builder::default()
      // The input header we would like to generate
      // bindings for.
      .header("wrapper.h")
      .clang_arg(&format!("-I{}/include/libprojectM", out_dir.display()))
      // Tell cargo to invalidate the built crate whenever any of the
      // included header files changed.
      .parse_callbacks(Box::new(bindgen::CargoCallbacks))
      // Finish the builder and generate the bindings.
      .generate()
      // Unwrap the Result and panic on failure.
      .expect("Unable to generate bindings");

  bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");
}