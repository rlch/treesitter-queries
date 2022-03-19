use std::{
    ffi::OsStr,
    fs::canonicalize,
    path::{Path, PathBuf},
};

fn main() {
    setup("yaml");
}

fn setup(lang: &str) {
    let package = format!("tree-sitter-{}", lang);
    let src_path: &Path = &canonicalize(
        &[".", "parsers", &package, "src"]
            .iter()
            .collect::<PathBuf>(),
    )
    .expect("Could not find src path");

    let parser_path = src_path.join("parser.c");
    let mut scanner_path = src_path.join("scanner.c");

    let scanner_path = if scanner_path.exists() {
        Some(scanner_path)
    } else {
        scanner_path.set_extension("cc");
        if scanner_path.exists() {
            Some(scanner_path)
        } else {
            None
        }
    };

    println!("cargo:rerun-if-changed={}", src_path.to_str().unwrap());

    match &scanner_path {
        Some(scanner_path) => match scanner_path.extension().and_then(OsStr::to_str) {
            Some("c") => {
                cc::Build::new()
                    .cpp(false)
                    .include(&src_path)
                    .file(&parser_path)
                    .file(&scanner_path)
                    .warnings(false)
                    .compile(&package);
            }
            Some("cc") => {
                cc::Build::new()
                    .cpp(true)
                    .include(&src_path)
                    .file(&scanner_path)
                    .warnings(false)
                    .compile(&format!("{}_scanner", package));
                cc::Build::new()
                    .cpp(false)
                    .include(&src_path)
                    .file(&parser_path)
                    .warnings(false)
                    .compile(&format!("{}_parser", package));
            }
            _ => panic!("Unknown extension for scanner"),
        },
        None => {
            cc::Build::new()
                .cpp(false)
                .include(&src_path)
                .file(&parser_path)
                .warnings(false)
                .compile(&package);
        }
    }
}
