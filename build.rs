#[cfg(not(windows))]
use std::env::consts::OS;
#[cfg(windows)]
use winres::WindowsResource;

#[cfg(windows)]
fn append_resource() {
    let mut res = WindowsResource::new();

    res.set_manifest(
        r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#,
    );

    res.compile().unwrap();
}

#[cfg(not(windows))]
fn append_resource() {
    println!("cargo:warning=This is not a windows system! (system is {}) The program may not work as expected", OS);
}

fn main() {
    append_resource()
}
