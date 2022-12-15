use std::{path::{Path, PathBuf}, env};

pub fn bindgen() {
  println!("cargo:rerun-if-changed=wrapper.h");
  
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let out_path = Path::new(".").join("src");
  
  // Get header based on features
  fn get_header() -> String {
    if cfg!(feature = "enable-playlist") {
      return "bindgen/playlist.h".to_string().to_string()
    } else {
      return "bindgen/default.h".to_string().to_string()
    }
  }

  let bindings = bindgen::Builder::default()
      .header(get_header())
      .allowlist_function("projectm_.*")
      .clang_arg(&format!("-I{}/include", out_dir.display()))
      .parse_callbacks(Box::new(bindgen::CargoCallbacks))
      .generate()
      .expect("Unable to generate bindings");

  bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");
}