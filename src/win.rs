#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(target_os = "windows")]
use winreg::enums::*;

#[derive(Debug)]
pub struct WindowsSystemInformation {
    pub product_key: String,
    pub product_name: String,
    pub registered_owner: String,
    pub installed_date: u64,
    pub installed_time: u64,
}

#[cfg(target_os = "windows")]
pub fn get_information() -> WindowsSystemInformation {
    let reg = RegKey::predef(HKEY_CURRENT_MACHINE);
    let current_version = reg
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")
        .expect(
            "Expected for SOFTWARE\\Microsoft\\Windows\\CurrentVersion to exist or to be opened",
        );

    let prod_key = get_product_key(&current_version);
    let prod_name = get_product_name(&current_version);
    let owner = get_registered_owner(&current_version);
    let (inst_date, inst_time) = get_install_datetime(&current_version);

    WindowsSystemInformation {
        product_key: prod_key,
        product_name: prod_name,
        registered_owner: owner,
        installed_date: inst_date,
        installed_time: inst_time,
    }
}

#[cfg(target_os = "windows")]
fn get_product_key(current: &RegKey) -> String {
    current
        .open_subkey("SoftwareProtectionPlatform")
        .expect("Expected subkey SoftwareProtectionPlatform to exist")
        .get_value("BackupProductKeyDefault")
        .expect("Expected key BackupProductKeyDefault to exist")
}

#[cfg(target_os = "windows")]
fn get_product_name(current: &RegKey) -> String {
    current
        .get_value("ProductName")
        .expect("Expected key ProductName to exist")
}

#[cfg(target_os = "windows")]
fn get_registered_owner(current: &RegKey) -> String {
    current
        .get_value("RegisteredOwner")
        .expect("Expected key RegisteredOwner to exist")
}

#[cfg(target_os = "windows")]
fn get_install_datetime(current: &RegKey) -> (u64, u64) {
    (
        current.get_value("InstallDate"),
        current.get_value("InstallTime"),
    )
}
