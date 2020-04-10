extern crate bindgen;
extern crate cc;

use std::{
    env::{self, VarError},
    ffi::OsStr,
    iter,
    path::PathBuf,
};

fn main() {
    println!("cargo:rustc-link-lib=SFCGAL");

    let mut cargo_metadata = Vec::with_capacity(64);
    let link_libs_opt = env_var("SFCGAL_LINK_LIBS");
    let link_paths_opt = env_var("SFCGAL_LINK_PATHS");
    let include_paths_opt = env_var("SFCGAL_INCLUDE_PATHS");

    if let Some(link_paths) = link_paths_opt {
        let meta = link_paths
            .split(',')
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(|path| {
                let out = iter::once(format!("cargo:rustc-link-search={}", path));
                #[cfg(target_os = "macos")]
                {
                    out.chain(iter::once(format!(
                        "cargo:rustc-link-search=framework={}",
                        path
                    )))
                }
                #[cfg(not(target_os = "macos"))]
                {
                    out
                }
            })
            .flatten();

        cargo_metadata.extend(meta);
    }

    if let Some(link_libs) = link_libs_opt {
        cargo_metadata.extend(
            process_library_list(
                link_libs
                    .split(',')
                    .map(str::trim)
                    .filter(|x| !x.is_empty()),
            )
            .map(|l| format!("cargo:rustc-link-lib={}", l)),
        );
    }

    let include_paths: Vec<_> = match include_paths_opt {
        Some(include_paths) => include_paths
            .split(',')
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(PathBuf::from)
            .collect(),
        None => vec![PathBuf::from("/usr/include")],
    };

    {
        let mut build = cc::Build::new();
        build.file("src/wrapper.c");

        for include_path in include_paths.iter() {
            build.include(include_path);
        }

        build.compile("sfcgalwrapper");
    }

    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Stable_1_33)
        .header("src/wrapper.h")
        .clang_args(
            include_paths
                .iter()
                .map(|path| format!("-I{}", path.display())),
        )
        .whitelist_type("sfcgal_.*$")
        .whitelist_var("sfcgal_.*$")
        .whitelist_function("sfcgal_.*$")
        .whitelist_type("w_sfcgal_.*$")
        .whitelist_var("w_sfcgal_.*$")
        .whitelist_function("w_sfcgal_.*$")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    for meta in cargo_metadata.into_iter() {
        println!("{}", meta);
    }
}

fn process_library_list(
    libs: impl IntoIterator<Item = impl Into<PathBuf>>,
) -> impl Iterator<Item = String> {
    libs.into_iter().map(|x| {
        let mut path: PathBuf = x.into();
        let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
        let is_framework = extension.eq_ignore_ascii_case("framework");
        const LIB_EXTS: [&str; 7] = ["so", "a", "dll", "lib", "dylib", "framework", "tbd"];
        if is_framework || LIB_EXTS.iter().any(|e| e.eq_ignore_ascii_case(extension)) {
            path.set_extension("");
        }
        path.file_name()
            .and_then(|f| {
                f.to_str().map(|f| {
                    if is_framework {
                        format!("framework={}", f)
                    } else {
                        f.to_owned()
                    }
                })
            })
            .expect("Invalid library name")
    })
}

fn env_var<K: AsRef<OsStr>>(key: K) -> Option<String> {
    match env::var(key) {
        Ok(val) => Some(val),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(_)) => panic!("the value of environment variable is not Unicode"),
    }
}
