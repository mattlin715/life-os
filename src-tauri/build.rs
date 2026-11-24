#[path = "build_support/pilot_credentials_inputs.rs"]
mod pilot_credentials_inputs;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android")
        && std::env::var_os("CARGO_FEATURE_ANDROID_PILOT_ONLINE").is_some()
    {
        let manifest = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
        let android = std::path::PathBuf::from(std::env::var_os("TAURI_ANDROID_PROJECT_PATH")
            .expect("Connected Pilot requires an explicit isolated Android project"));
        println!("cargo:rerun-if-env-changed=TAURI_ANDROID_PROJECT_PATH");
        for name in pilot_credentials_inputs::CREDENTIAL_INPUTS {
            println!("cargo:rerun-if-changed=android-pilot-online/java/com/lifeos/pilot/credentials/{name}");
            println!("cargo:rerun-if-changed={}", android.join("app/src/main/java/com/lifeos/pilot/credentials").join(name).display());
        }
        pilot_credentials_inputs::verify(&manifest, &android)
            .expect("Stage the exact connected Pilot credential Java sources before compiling; do not reuse a 3101 scaffold without them");
    }
    tauri_build::build();
}
