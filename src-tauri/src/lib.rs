#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
mod filesystem_safety;
#[cfg(all(not(target_os = "android"), any(feature = "desktop-schema-v5", feature = "founder-schema-v5")))]
mod schema_v5_founder_activation;
#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
mod schema_v5_migration;
#[cfg(all(not(target_os = "android"), feature = "desktop-schema-v5"))]
mod schema_v5_prepared_recovery;
#[cfg(not(target_os = "android"))]
mod sqlite;

#[cfg(not(target_os = "android"))]
mod desktop_runtime;

#[cfg(not(target_os = "android"))]
pub fn run() {
    desktop_runtime::run();
}

#[cfg(target_os = "android")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running the isolated Life OS Android feasibility M0 shell");
}
