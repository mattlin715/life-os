//! Native-only mobile vault. No renderer invoke handler or credential getter is exposed.
#[cfg(target_os = "android")]
use tauri::{plugin::{Builder, PluginHandle, TauriPlugin}, Manager, Wry};

#[cfg(target_os = "android")]
struct CredentialBridge(PluginHandle<Wry>);

#[cfg(target_os = "android")]
pub(crate) fn init() -> TauriPlugin<Wry> {
    Builder::new("pilot-credentials")
        .setup(|app, api| {
            let handle = api.register_android_plugin("com.lifeos.pilot.credentials", "PilotCredentialsPlugin")?;
            app.manage(CredentialBridge(handle));
            Ok(())
        })
        .build()
}

pub(crate) fn load(app: &tauri::AppHandle) -> Result<Option<String>, &'static str> {
    #[cfg(target_os = "android")]
    {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Loaded { key: Option<String> }
        app.state::<CredentialBridge>().0
            .run_mobile_plugin::<Loaded>("loadKey", serde_json::json!({}))
            .map(|value| value.key)
            .map_err(|_| "credential_unavailable")
    }
    #[cfg(not(target_os = "android"))]
    { let _ = app; Ok(None) }
}

pub(crate) fn replace(app: &tauri::AppHandle, key: Option<&str>) -> Result<(), &'static str> {
    #[cfg(target_os = "android")]
    {
        app.state::<CredentialBridge>().0
            .run_mobile_plugin::<()>("replaceKey", serde_json::json!({ "key": key }))
            .map_err(|_| "credential_unavailable")
    }
    #[cfg(not(target_os = "android"))]
    { let _ = app; if key.is_some() { Err("credential_unavailable") } else { Ok(()) } }
}
