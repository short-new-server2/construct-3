//! Windows registry read helpers for Construct 3.

pub const APP_KEY: &str = "Software\\Construct\\Construct 3";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Construct 3";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Construct 3") },
        RegValue { name: "Version",     value: String::from("v2.6.13") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Construct 3") },
        RegValue { name: "Publisher",       value: String::from("Construct") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Construct 3\\uninstall.exe\"") },
    ]
}
