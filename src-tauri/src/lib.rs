#[cfg(any(target_os = "android", test))]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
mod android_m1;
#[cfg(any(target_os = "android", test))]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
mod android_m2a;
#[cfg(any(target_os = "android", test))]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
mod android_m2b;
#[cfg(any(target_os = "android", test))]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
mod android_m2c;
#[cfg(all(any(target_os = "android", test), feature = "android-pilot-online"))]
mod android_pilot_model;
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
#[cfg(all(test, not(target_os = "android"), feature = "founder-schema-v5"))]
#[path = "../tests/diagnostics/reflection_generation_capture.rs"]
mod reflection_generation_capture;
#[cfg(all(test, not(target_os = "android")))]
#[path = "../tests/diagnostics/reflection_generation_additions.rs"]
mod reflection_generation_additions;
#[cfg(all(test, not(target_os = "android")))]
#[path = "../tests/diagnostics/reflection_projection_ordering.rs"]
mod reflection_projection_ordering;
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

#[cfg(all(target_os = "android", feature = "android-pilot", debug_assertions))]
compile_error!("The Android pilot must be built without debug assertions.");

#[cfg(target_os = "android")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();
    #[cfg(feature = "android-pilot-online")]
    let builder = builder.manage(android_pilot_model::ModelSession::default());
    builder
        .invoke_handler(tauri::generate_handler![
            #[cfg(not(feature = "android-pilot"))]
            android_m2a::m2a_storage_status,
            #[cfg(not(feature = "android-pilot"))]
            android_m2a::m2a_create_experience,
            #[cfg(not(feature = "android-pilot"))]
            android_m2a::m2a_list_experiences,
            #[cfg(not(feature = "android-pilot"))]
            android_m2a::m2a_get_experience,
            #[cfg(not(feature = "android-pilot"))]
            android_m2a::m2a_get_locale_preference,
            #[cfg(not(feature = "android-pilot"))]
            android_m2a::m2a_set_locale_preference,
            #[cfg(not(feature = "android-pilot"))]
            android_m2b::m2b_storage_status,
            #[cfg(not(feature = "android-pilot"))]
            android_m2b::m2b_create_experience,
            #[cfg(not(feature = "android-pilot"))]
            android_m2b::m2b_list_experiences,
            #[cfg(not(feature = "android-pilot"))]
            android_m2b::m2b_get_experience,
            #[cfg(not(feature = "android-pilot"))]
            android_m2b::m2b_mutate_experience,
            #[cfg(not(feature = "android-pilot"))]
            android_m2b::m2b_get_locale_preference,
            #[cfg(not(feature = "android-pilot"))]
            android_m2b::m2b_set_locale_preference,
            android_m2c::m2c_storage_status,
            android_m2c::m2c_create_experience,
            android_m2c::m2c_list_experiences,
            android_m2c::m2c_get_experience,
            android_m2c::m2c_mutate_experience,
            android_m2c::m2c_get_locale_preference,
            android_m2c::m2c_set_locale_preference,
            android_m2c::m2c_artifact_snapshot,
            android_m2c::m2c_generate_questions,
            android_m2c::m2c_mutate_artifact,
            #[cfg(feature = "android-pilot-online")]
            android_pilot_model::pilot_model_has_key,
            #[cfg(feature = "android-pilot-online")]
            android_pilot_model::pilot_model_set_key,
            #[cfg(feature = "android-pilot-online")]
            android_pilot_model::pilot_model_clear_key,
            #[cfg(feature = "android-pilot-online")]
            android_pilot_model::pilot_model_ask,
            #[cfg(feature = "android-pilot-online")]
            android_pilot_model::pilot_model_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the isolated Life OS Android M2-A direct-v5 review app");
}
