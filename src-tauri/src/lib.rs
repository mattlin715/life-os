#[allow(dead_code)]
mod filesystem_safety;
#[cfg(all(not(target_os = "android"), any(feature = "desktop-schema-v5", feature = "founder-schema-v5")))]
mod schema_v5_founder_activation;
#[allow(dead_code)]
mod schema_v5_migration;
mod schema_v5_fresh_base;
#[cfg(any(target_os = "android", test))]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
mod android_m1;
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
        .invoke_handler(tauri::generate_handler![
            android_m1::m1_storage_status,
            android_m1::m1_create_experience,
            android_m1::m1_list_experiences,
            android_m1::m1_get_experience,
            android_m1::m1_get_locale_preference,
            android_m1::m1_set_locale_preference,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the isolated Life OS Android M1 persistence review app");
}
