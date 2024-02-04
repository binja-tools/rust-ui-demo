# Binary Ninja Sidebar UI Example

Example project containing a binary ninja sidebar written in Rust.

## Installation

You need to have Qt installed on your machine.
Also, `qmake` has to be available, either in your Path env or via the QMAKE env
variable.
Adding `C:\Qt\6.6.1\msvc2019_64\bin` to the path on windows seems to work.
Linux distributions make it available via package manager.

### 1. Install rust

https://rustup.rs/

### 2. Update deps

If you are building for the stable release, uncomment the `branch` fields in `Cargo.toml`.

Make sure you build against the latest version of the binja api:

```sh
cargo update
```

### 3. Build

```sh
cargo build --release
```

### 4. Link to binja plugin folder

#### Linux
```sh
ln -s ${PWD}/target/release/libbinja_diff.so ~/.binaryninja/plugins/
```

#### Windows
##### CMD
```cmd
mklink "%APPDATA%\Binary Ninja\plugins\binja_diff.dll" "%CD%\target\release\binja_diff.dll"
```
##### POWERSHELL
```ps1
New-Item -ItemType SymbolicLink -Path "$env:APPDATA\Binary Ninja\plugins\binja_diff.dll" -Target "$PWD\target\release\binja_diff.dll"
```
