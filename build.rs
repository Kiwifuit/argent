#[cfg(windows)]
use winres::WindowsResource;
#[cfg(not(windows))]
use std::env::consts::OS;

#[cfg(windows)]
const MANIFEST_FILE = include_str!("./appmanifest.xml");

#[cfg(windows)]
fn append_resource() {
    let mut res = WindowsResource::new();

    res.compile().unwrap();
}

#[cfg(not(windows))]
fn append_resource() {
    println!("cargo:warning=This is not a windows system! (system is {}) The `appmanifest.xml` file will not be added onto the executable", OS);
}

fn main() {
    append_resource()
}