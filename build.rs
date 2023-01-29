#[macro_use]
extern crate lazy_static;

use std::{env, path::Path, process::Command};

mod build_bindgen;
use crate::build_bindgen::bindgen;

lazy_static! {
    static ref PROJECTM_BUILD: String = format!("{}/projectm", env::var("OUT_DIR").unwrap());
}

fn main() {
    if !Path::new(PROJECTM_BUILD.as_str()).exists() {
        let _ = Command::new("git")
            .args([
                "clone",
                "--depth=1",
                "--branch",
                "v4.0.0-rc1",
                "https://github.com/projectM-visualizer/projectm.git",
                &PROJECTM_BUILD,
            ])
            .status();
    }

    // Feature: enable-playlist
    fn enable_playlist() -> String {
        if cfg!(feature = "playlist") {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    #[cfg(target_os = "windows")]
    let dst = cmake::Config::new(PROJECTM_BUILD.as_str())
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .build();

    #[cfg(target_os = "linux")]
    let dst = cmake::Config::new(PROJECTM_BUILD.as_str())
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .build();

    #[cfg(target_os = "macos")]
    let dst = cmake::Config::new(PROJECTM_BUILD.as_str())
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .build();

    #[cfg(target_os = "emscripten")]
    let dst = cmake::Config::new(PROJECTM_BUILD.as_str())
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .define("ENABLE_EMSCRIPTEN", "ON")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    #[cfg(target_os = "windows")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=dylib=projectM");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlist");
    } else {
        println!("cargo:rustc-link-lib=dylib=projectMd");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlistd");
    }

    #[cfg(target_os = "linux")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=dylib=projectM");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlist");
    } else {
        println!("cargo:rustc-link-lib=dylib=projectMd");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlistd");
    }

    #[cfg(target_os = "macos")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=dylib=projectM");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlist");
    } else {
        println!("cargo:rustc-link-lib=dylib=projectMd");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlistd");
    }

    #[cfg(target_os = "emscripten")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=static=projectM");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlist");
    } else {
        println!("cargo:rustc-link-lib=static=projectMd");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM_playlistd");
    }

    bindgen()
}
