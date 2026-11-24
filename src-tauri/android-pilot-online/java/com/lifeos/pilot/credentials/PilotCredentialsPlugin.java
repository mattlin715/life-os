package com.lifeos.pilot.credentials;

import android.app.Activity;
import app.tauri.annotation.Command;
import app.tauri.annotation.TauriPlugin;
import app.tauri.plugin.Invoke;
import app.tauri.plugin.JSObject;
import app.tauri.plugin.Plugin;
import org.json.JSONObject;

/** Invoked by the native Rust bridge only; no frontend credential getter/permission is registered. */
@TauriPlugin
public final class PilotCredentialsPlugin extends Plugin {
    private final CredentialStore store;

    public PilotCredentialsPlugin(Activity activity) {
        super(activity);
        store = new CredentialStore(new AndroidCredentialBackend(activity));
    }

    @Command public void loadKey(Invoke invoke) {
        try {
            if (invoke.getArgs().length() != 0) throw new Exception();
            String key = store.load();
            JSObject result = new JSObject();
            result.put("key", key == null ? JSONObject.NULL : key);
            invoke.resolve(result);
        } catch (Exception error) {
            // Never forward the exception: Tauri logs exception objects when supplied to reject.
            invoke.reject("credential_unavailable");
        }
    }

    @Command public void replaceKey(Invoke invoke) {
        try {
            JSObject args = invoke.getArgs();
            if (args.length() != 1 || !args.has("key")) throw new Exception();
            String key = args.isNull("key") ? null : args.getString("key");
            store.replace(key);
            invoke.resolve();
        } catch (Exception error) {
            invoke.reject("credential_unavailable");
        }
    }
}
