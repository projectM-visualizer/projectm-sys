use std::{env, path::Path, process::Command};

mod build_bindgen;
use crate::build_bindgen::bindgen;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=wrapper.h");

  if !Path::new("projectm/src").exists() {
    let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
  }

  #[cfg(target_os = "windows")]
  let dst = cmake::build("projectm");
  
  #[cfg(target_os = "linux")]
  let dst = cmake::build("projectm");

  #[cfg(target_os = "ios")]
  let dst = cmake::build("projectm");

  #[cfg(target_os = "emscripten")]
  let dst = cmake::build("projectm");

  println!("cargo:rustc-link-search=native={}/lib", dst.display());

  #[cfg(target_os = "windows")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }

  #[cfg(target_os = "linux")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }

  #[cfg(target_os = "ios")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }
  
  #[cfg(target_os = "emscripten")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=static=projectM");
  } else {
    println!("cargo:rustc-link-lib=static=projectMd");
  }
  
  bindgen()
}



