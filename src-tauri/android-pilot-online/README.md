# Connected Pilot native credential overlay

Only the separately selected `android-pilot-online` feature registers
`android_pilot_credentials`. It attaches the Java class
`com.lifeos.pilot.credentials.PilotCredentialsPlugin`; no renderer credential
getter or plugin permission is added. Rust credential commands use a blocking
worker to wait for the mobile bridge, not the WebView IPC/UI thread.

The three Java sources under `java/com/lifeos/pilot/credentials/` must be copied
as ordinary files into the **new isolated** Gradle project's matching
`app/src/main/java/` directory before the locked Cargo compilation. The
build-support guard checks exact input bytes and refuses missing, linked or
stale sources. Do not modify an accepted 3101 package, reuse generated classes
from it, or run an old standalone recipe without adapting its source manifest.
The release manifest overlay remains required separately.

Key behavior:

- Default is session-only. Opt-in uses an Android Keystore AES-GCM wrapping
  key with randomized IVs and app/version-bound AAD; only ciphertext is saved
  in `Context.getNoBackupFilesDir()` via `AtomicFile`.
- Forget/session-only writes an empty disabled marker before removing the
  wrapping alias. A cleanup failure is visible; the disabled marker prevents
  the old ciphertext from being restored if alias deletion fails.
- A failed atomic write does not claim successful forgetting. A key could
  still remain saved if writing the disabled marker itself fails; the user
  is explicitly told to retry clearing, never shown false success.
- No hardware-backed-key guarantee is inferred from the phone model. No
  plaintext fallback, regeneration on decryption, logging of exceptions or
  automatic restoration of a failed credential is implemented.
- Clearing is local, not revocation at Google. Uninstall/data-clear/device
  replacement/key invalidation require re-entry. Update continuity must be
  verified on-device after a separately approved package.

Run `scripts/test-android-pilot-credentials.ps1` with explicit JDK, Android API
jar, existing compiled Tauri classes, Kotlin libraries and a new output path.
It runs a pure Java injected-backend lifecycle suite and compiles the Android
adapter. This is **not** a Gradle/APK build and does not access Android
Keystore, a real API key, a signing key, any phone or emulator.

Journal schema is unchanged. Feedback save and supplement turns are not
implemented: see proposed ADR-0013 and the R3 product proposal. Do not add
JSON/prefs storage for reflection content to get around that gate.

References: [Android Keystore](https://developer.android.com/privacy-and-security/keystore),
[AtomicFile](https://developer.android.com/reference/android/util/AtomicFile),
[no-backup directory](https://developer.android.com/reference/android/content/Context#getNoBackupFilesDir()).
