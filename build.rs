use std::{env, path::Path, process::Command};

mod build_bindgen;
use crate::build_bindgen::bindgen;

fn main() {
  if !Path::new("projectm/src").exists() {
    let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
  }

  // Feature: enable-playlist
  fn enable_playlist() -> String {
    if cfg!(feature = "playlist") {
      return "ON".to_string().to_string()
    } else {
      return "OFF".to_string().to_string()
    }
  }
  
  #[cfg(target_os = "windows")]
  let dst = cmake::Config::new("projectm")
                          .define("ENABLE_PLAYLIST", enable_playlist().as_str())
                          .build(); 

  #[cfg(target_os = "linux")]
  let dst = cmake::Config::new("projectm")
                          .define("ENABLE_PLAYLIST", enable_playlist().as_str())
                          .build();

  #[cfg(target_os = "ios")]
  let dst = cmake::Config::new("projectm")
                          .define("ENABLE_PLAYLIST", enable_playlist().as_str())
                          .build();

  #[cfg(target_os = "emscripten")]
  let dst = cmake::Config::new("projectm")
                          .define("ENABLE_PLAYLIST", enable_playlist().as_str())
                          .define("ENABLE_EMSCRIPTEN", "ON")
                          .build();

  println!("cargo:rustc-link-search=native={}/lib", dst.display());

  #[cfg(target_os = "windows")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlist");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlistd");
  }

  #[cfg(target_os = "linux")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlist");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlistd");
  }

  #[cfg(target_os = "ios")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlist");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlistd");
  }
  
  #[cfg(target_os = "emscripten")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=static=projectM");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlist");
  } else {
    println!("cargo:rustc-link-lib=static=projectMd");

    #[cfg(feature = "playlist")]
    println!("cargo:rustc-link-lib=dylib=libprojectM_playlistd");
  }
  
  bindgen()
}



