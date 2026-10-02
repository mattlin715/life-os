#[cfg(any(target_os = "android", test))]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
mod android_m1;
#[cfg(any(target_os = "android", test))]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
mod android_m2a;
#[allow(dead_code)]
mod filesystem_safety;
#[cfg(all(
    not(target_os = "android"),
    any(feature = "desktop-schema-v5", feature = "founder-schema-v5")
))]
mod schema_v5_founder_activation;
mod schema_v5_fresh_base;
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
        .invoke_handler(tauri::generate_handler![
            android_m2a::m2a_storage_status,
            android_m2a::m2a_create_experience,
            android_m2a::m2a_list_experiences,
            android_m2a::m2a_get_experience,
            android_m2a::m2a_get_locale_preference,
            android_m2a::m2a_set_locale_preference,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the isolated Life OS Android M2-A direct-v5 review app");
}
