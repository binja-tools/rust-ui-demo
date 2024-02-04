use std::{
    env, fs,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use cargo_metadata::MetadataCommand;
use cxx_qt_build::CxxQtBuilder;

#[cfg(target_os = "macos")]
static LASTRUN_PATH: (&str, &str) = ("HOME", "Library/Application Support/Binary Ninja/lastrun");

#[cfg(target_os = "linux")]
static LASTRUN_PATH: (&str, &str) = ("HOME", ".binaryninja/lastrun");

#[cfg(windows)]
static LASTRUN_PATH: (&str, &str) = ("APPDATA", "Binary Ninja\\lastrun");

// Check last run location for path to BinaryNinja; Otherwise check the default
// install locations
fn link_path() -> PathBuf {
    use std::io::prelude::*;

    let home = PathBuf::from(env::var(LASTRUN_PATH.0).unwrap());
    let lastrun = PathBuf::from(&home).join(LASTRUN_PATH.1);

    File::open(lastrun)
        .and_then(|f| {
            let mut binja_path = String::new();
            let mut reader = BufReader::new(f);

            reader.read_line(&mut binja_path)?;
            Ok(PathBuf::from(binja_path.trim()))
        })
        .unwrap_or_else(|_| {
            #[cfg(target_os = "macos")]
            return PathBuf::from("/Applications/Binary Ninja.app/Contents/MacOS");

            #[cfg(target_os = "linux")]
            return home.join("binaryninja");

            #[cfg(windows)]
            return PathBuf::from(env::var("PROGRAMFILES").unwrap())
                .join("Vector35\\BinaryNinja\\");
        })
}

fn main() {
    // Use BINARYNINJADIR first for custom BN builds/configurations (BN devs/build
    // server), fallback on defaults
    let install_path = env::var("BINARYNINJADIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| link_path());

    #[cfg(target_os = "linux")]
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{},-L{},-l:libbinaryninjacore.so.1",
        install_path.to_str().unwrap(),
        install_path.to_str().unwrap(),
    );

    #[cfg(target_os = "macos")]
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{},-L{},-lbinaryninjacore",
        install_path.to_str().unwrap(),
        install_path.to_str().unwrap(),
    );

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=binaryninjacore");
        println!("cargo:rustc-link-lib=binaryninjaui");
        println!("cargo:rustc-link-search={}", install_path.to_str().unwrap());
    }

    // Fetch the path to the binaryninja git checkout from cargo.
    // We use this to tell cxx-qt/cxx/cc about the header files for the binaryninja
    // libs.
    // Paths that are added to include path: `/`, `/ui`, `/vendor/fmt/include`.
    let metadata = MetadataCommand::new().exec().unwrap();
    let binja_source = &metadata
        .packages
        .iter()
        .find(|x| x.name == "binaryninja")
        .unwrap()
        .targets
        .iter()
        .find(|x| x.name == "binaryninja")
        .unwrap()
        .src_path;
    let binja_root = binja_source
        .as_std_path()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let binja_ui = binja_root.join("ui");
    let fmt = binja_root.join("vendor/fmt/include");

    let mut c_files = Vec::new();
    let mut rs_files = Vec::new();

    // Find all cpp and c files to build with cc
    // and find all rs files that contain a cxx bridge to build with cxx-qt
    fn path_search(
        path: impl AsRef<Path>,
        c_files: &mut Vec<PathBuf>,
        rs_files: &mut Vec<PathBuf>,
    ) {
        fs::read_dir(path)
            .unwrap()
            .filter_map(|f| f.ok())
            .for_each(|f| {
                let path = f.path();
                if path.is_dir() {
                    path_search(path, c_files, rs_files);
                    return;
                }
                let name = f.file_name();
                let name = name.to_str().unwrap();
                if name.ends_with(".cpp") || name.ends_with(".c") {
                    c_files.push(f.path());
                } else if name.ends_with(".rs") {
                    let content = fs::read_to_string(f.path()).unwrap();
                    if content.contains("#[cxx::bridge]") || content.contains("#[cxx_qt::bridge]") {
                        rs_files.push(f.path());
                    }
                }
            })
    }
    path_search("src/binja_ui_ffi", &mut c_files, &mut rs_files);

    // CxxQtBuilder finds Qt on its own.
    // You need to have Qt installed on your machine.
    // Also, qmake has to be available, either in your Path env or via the QMAKE env
    // variable.
    // Adding `C:\Qt\6.6.1\msvc2019_64\bin` to the path on windows seems to work.
    // Linux distributions make it available via package manager.
    // See more info about this: https://github.com/KDAB/cxx-qt/blob/c80e8b4d77ceb2ce7b8180dce3b9aa244cead083/crates/qt-build-utils/src/lib.rs#L165
    let mut qt_builder = CxxQtBuilder::new()
        .qt_module("Network")
        .qt_module("Widgets");

    for file in rs_files {
        qt_builder = qt_builder.file(file);
    }

    qt_builder
        .cc_builder(|cc| {
            cc.include(binja_root.to_str().unwrap());
            cc.include(binja_ui.to_str().unwrap());
            cc.include(fmt.to_str().unwrap());
            cc.include("src/binja_ui_ffi/cpp");
            cc.files(c_files.clone());
            cc.warnings(false);
        })
        .build();

    println!("cargo:rerun-if-changed=src/binja_ui_ffi/cpp");
}
