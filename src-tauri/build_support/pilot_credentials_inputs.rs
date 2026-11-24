use std::{fs, path::Path};

pub const CREDENTIAL_INPUTS: [&str; 3] = [
    "CredentialStore.java", "AndroidCredentialBackend.java", "PilotCredentialsPlugin.java",
];

// The standalone packager must copy these ordinary files before Cargo/Gradle.
// Otherwise Rust could register a class that never makes it into the APK.
pub fn verify(manifest: &Path, android: &Path) -> Result<(), &'static str> {
    let package = "com/lifeos/pilot/credentials";
    for name in CREDENTIAL_INPUTS {
        let input = manifest.join("android-pilot-online/java").join(package).join(name);
        let staged = android.join("app/src/main/java").join(package).join(name);
        for path in [&input, &staged] {
            if !fs::symlink_metadata(path).map_err(|_| "credential_source_missing")?.file_type().is_file() {
                return Err("credential_source_not_ordinary");
            }
        }
        if fs::read(input).map_err(|_| "credential_source_unreadable")?
            != fs::read(staged).map_err(|_| "credential_source_unreadable")? {
            return Err("credential_source_differs");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn model_trial_credential_source_staging_is_exact() {
        let root = tempfile::tempdir().unwrap();
        let manifest = root.path().join("source");
        let android = root.path().join("android");
        assert_eq!(verify(&manifest, &android), Err("credential_source_missing"));
        for name in CREDENTIAL_INPUTS {
            for parent in [manifest.join("android-pilot-online/java"), android.join("app/src/main/java")] {
                let dir = parent.join("com/lifeos/pilot/credentials");
                fs::create_dir_all(&dir).unwrap();
                fs::write(dir.join(name), format!("synthetic-{name}")).unwrap();
            }
        }
        assert!(verify(&manifest, &android).is_ok());
        fs::write(android.join("app/src/main/java/com/lifeos/pilot/credentials/CredentialStore.java"), "stale").unwrap();
        assert_eq!(verify(&manifest, &android), Err("credential_source_differs"));
    }
}
